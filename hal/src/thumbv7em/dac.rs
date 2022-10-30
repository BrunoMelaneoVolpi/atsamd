

use core::marker::PhantomData;

use crate::pac::{DAC, MCLK};
//use crate::pac::{DAC_OTHER, TC5};

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
            .cctrl().cc12m()                //  TBD...
            .fext().clear_bit()             //  TBD:    use "integrated" for now...
            .osr().osr_1());                //  TBD:    For now no oversampling/interpolation

        Self { _marker: PhantomData }
    }


    pub fn start_conversion(self, dac: &mut Dac) -> Self {

        dac.0.data[I::INDEX].write(|w| unsafe { w.bits(0x1234) });

        if 0 == I::INDEX {
            while dac.0.syncbusy.read().data0().bit_is_set() {}
        } else {
            while dac.0.syncbusy.read().data1().bit_is_set() {}
        }

        self
    }

}

impl Dac {
    pub fn init(mclk: &mut MCLK, mut dac: DAC) -> (Dac, DacUnit<Dac0>, DacUnit<Dac1>) {

        /*  Enable DAC main clock...    */
        mclk.apbdmask.modify(|_, w| w.dac_().set_bit());

        //  Reset just in case...
        dac.ctrla.modify(|_, w| w.swrst().set_bit());
        while dac.syncbusy.read().swrst().bit_is_set() {}

        let dac0: DacUnit<Dac0> = DacUnit::init(&mut dac);
        let dac1: DacUnit<Dac1> = DacUnit::init(&mut dac);
        (Self(dac), dac0, dac1)
    }

    pub fn enable_dac_controller(self) -> Self {
        /*  This enables the DAC controller which includes both
            DAC0 and DAC1.  */
        self.0.ctrla.modify(|_, w| w.enable().set_bit());
        while self.0.syncbusy.read().enable().bit_is_set() {}


        //  Once the DAC Controller is enabled, DACx requires a startup time
        //  before the first conversion can start. The DACx
        //  Startup Ready bit in the Status register (STATUS.READYx) indicates
        //  that DACx is ready to convert a data when
        //  STATUS.READYx=1.
        //  Conversions started while STATUS.READYx=0 shall be discarded.
    /*
        if STATUS.READYx
            start conversion
    */


        self
    }
}
