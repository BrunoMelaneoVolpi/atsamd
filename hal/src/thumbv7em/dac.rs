
use core::marker::PhantomData;
use core::panic;
//use crate::clock::{GenericClockController, ClockId};
use crate::clock::GenericClockController;

use crate::pac::gclk::pchctrl::GEN_A::GCLK11;
use crate::pac::gclk::genctrl::SRC_A::DFLL;

use crate::pac::{DAC, MCLK};
//use crate::pac::{DAC_OTHER, TC5};
use crate::pac::interrupt;
use cortex_m::peripheral::NVIC;


pub trait DacInstance {
    const INDEX: usize;
}
pub enum Dac0 {}
impl DacInstance for Dac0 {
    const INDEX: usize = 0;
}
pub enum Dac1 {}
impl DacInstance for Dac1 {
    const INDEX: usize = 1;
}

pub struct Dac(DAC);

// Dac::init(DAC) -> Dac<No, No>
// let dac: Dac<No, No> = Dac::init(DAC);
//   enable_dac_0<D1>(Dac<No, D1>, params)  -> (Dac<Yes, D1>, DacUnit<Dac0>)
// let (dac: Dac<Yes, No>, dac0: DacUnit<Dac0>) = dac.enable_dac_0(dac, init parameters);
//   enable_dac_1<D0>(Dac<No, D0>, params)  -> (Dac<Yes, D0>, DacUnit<Dac1>)
// let (dac: Dac<Yes, Yes>, dac1: DacUnit<Dac1>) = dac.enable_dac_1(dac, init parameters);
pub struct DacUnit<I: DacInstance> {
    _marker: PhantomData<I>
}

impl<I: DacInstance> DacUnit<I> {
    fn init(dac: &mut DAC) -> Self {
        ////////////////////////////////////
        //      47.6.2.3 DAC Configuration
        dac.dacctrl[I::INDEX].modify(|_, w| w
            .enable().set_bit()
            .leftadj().clear_bit()
            .runstdby().clear_bit()
            .dither().clear_bit()
            .refresh().refresh_1()
            .cctrl().cc100k()               //  TBD:    /!\ Depends on the CLK_DAC
            .fext().clear_bit()             //  TBD:    use "integrated" for now...
            .osr().osr_1());                //  TBD:    For now no oversampling/interpolation

        Self { _marker: PhantomData }
    }


    pub fn start_conversion(mut self, dac: &mut Dac, data: u16) -> Self {

        //if data > 4095{
        if data >= u16::pow(2, 12){
            panic!("DAC data out of 12 bit range: expected range [0...4095] received {}", data);
        }

        /*  Should we wait for the DAC to be ready? */
        self = self.wait_ready(dac);

        dac.0.data[I::INDEX].write(|w| unsafe { w.bits(data) });

        if 0 == I::INDEX {
            /*  Wait for the sync of the new value loaded in DATA...*/
            while dac.0.syncbusy.read().data0().bit_is_set() {}
            /*  ... and then wait for the EOC (end of conversion).  */
            //  Perhaps there is a better way to wait for the conversion completion and consequente VOUT0 is stabilisation.
            while dac.0.status.read().eoc0().bit_is_clear() {}
        } else {
            /*  Wait for the sync of the new value loaded in DATA...*/
            while dac.0.syncbusy.read().data1().bit_is_set() {}
            /*  ... and then wait for the EOC (end of conversion).  */
            //  Perhaps there is a better way to wait for the conversion completion and consequente VOUT0 is stabilisation.
            while dac.0.status.read().eoc1().bit_is_clear() {}
        }

        self
    }

    pub fn wait_ready(self, dac: &mut Dac) -> Self {
        /*  Once the DAC Controller is enabled, DACx requires a startup time
            before the first conversion can start. The DACx
            Startup Ready bit in the Status register (STATUS.READYx) indicates
            that DACx is ready to convert a data when
            STATUS.READYx=1.
            Conversions started while STATUS.READYx=0 shall be discarded. */

        /*  NOT SURE we should trap the processor here waiting for the
            unit to be ready... but maybe it is ok if is only done once
            during initialization.      */

        if 0 == I::INDEX {
            while dac.0.status.read().ready0().bit_is_clear(){}
        } else {
            while dac.0.status.read().ready1().bit_is_clear(){}
        }

        self
    }
}

impl Dac {
    pub fn init(mclk: &mut MCLK,
                mut dac: DAC,
                clocks: &mut GenericClockController) -> (Dac, DacUnit<Dac0>, DacUnit<Dac1>) {

        /*  Enable DAC main clock...    */
        mclk.apbdmask.modify(|_, w| w.dac_().set_bit());

        /*   */
        let dac_clock =
            clocks.configure_gclk_divider_and_source(GCLK11,
                1,
                DFLL,
                false).expect("GCLK11 clock setup failed");
        clocks.dac(&dac_clock).expect("dac clock setup failed");


        //  Reset just in case...
        dac.ctrla.modify(|_, w| w.swrst().set_bit());
        while dac.syncbusy.read().swrst().bit_is_set() {}

        //  The voltage reference is selected by writing to
        //  the Reference Selection bits in the Control B
        //  register (CTRLB.REFSEL).
        dac.ctrlb.modify(|_, w| w
            .diff().clear_bit()     //  Single mode
            .refsel().vddana());    //  Internal reference

        let dac0: DacUnit<Dac0> = DacUnit::init(&mut dac);
        let dac1: DacUnit<Dac1> = DacUnit::init(&mut dac);
        (Self(dac), dac0, dac1)
    }

    pub fn enable_dac_controller(self) -> Self {
        /*  This enables the DAC controller which includes both
            DAC0 and DAC1.  */
        self.0.ctrla.modify(|_, w| w.enable().set_bit());
        while self.0.syncbusy.read().enable().bit_is_set() {}

        self
    }

    pub fn enable_dac_interrupts(self, nvic: &mut NVIC) -> Self {
        /*  Enable all interrupts for now...    */

        //  Configure DAC interrupts at global interrupt controller
        unsafe {
            nvic.set_priority(interrupt::DAC_OTHER     , 1);
            nvic.set_priority(interrupt::DAC_EMPTY_0   , 1);
            nvic.set_priority(interrupt::DAC_EMPTY_1   , 1);
            nvic.set_priority(interrupt::DAC_RESRDY_0  , 1);
            nvic.set_priority(interrupt::DAC_RESRDY_1  , 1);

            NVIC::unmask(interrupt::DAC_OTHER   );
            NVIC::unmask(interrupt::DAC_EMPTY_0 );
            NVIC::unmask(interrupt::DAC_EMPTY_1 );
            NVIC::unmask(interrupt::DAC_RESRDY_0);
            NVIC::unmask(interrupt::DAC_RESRDY_1);
        }

        //  Configure DAC interrupts at DAC reguister level
        self.0.intenset.modify(
            |_, w| w
            .overrun0().set_bit()
            .overrun1().set_bit()
            .resrdy0().set_bit()
            .resrdy1().set_bit()
            .empty0().set_bit()
            .empty1().set_bit()
            .underrun0().set_bit()
            .underrun1().set_bit());

        self
    }
}
