
use atsamd51j::trng::evctrl::EVCTRL_SPEC;

use crate::pac::{MCLK, DAC};
//use crate::pac::{DAC_OTHER, TC5};


pub struct Dac(DAC);

impl Dac {
    pub fn init(mclk: &mut MCLK, dac: DAC) -> Dac {
        //**************************************************************** */
        //  Todo: How to make function generic to handle both DAC0 and DAC1?
        //  HACK:
        let dac0: usize = 0;
        let dac1: usize = 1;
        //**************************************************************** */


        /*  Enable DAC main clock...    */
        mclk.apbdmask.modify(|_, w| w.dac_().set_bit());

        //  Reset just in case...
        dac.ctrla.modify(|_, w| w.swrst().set_bit());
        while dac.syncbusy.read().swrst().bit_is_set() {};

//        47.6.2.3 DAC Configuration
//        *   Enable the selected DAC by writing a '1' to DACCTRLx.ENABLE
        dac.dacctrl[dac0].modify(|_, w| w.enable().set_bit());

//        *   Select the data alignment with DACCCTRLx.LEFTADJ. Writing a '1' will left-align the data (DATABUFx/
//            DATAx[31:20]). Writing a '0' to LEFTADJ will right-align the data (DATABUFx/DATAx[11:0]).
        dac.dacctrl[dac0].modify(|_, w| w.leftadj().clear_bit());

//        *   If operation in standby mode is desired for DACx, write a '1' to the Run in Standby bit in the DAC Control register
//            (DACCCTRLx.RUNSTDBY). If RUNSTDBY=1, DACx continues normal operation when system is in standby
//            mode. If RUNSTDBY=0, DACx is halted in standby mode.
        dac.dacctrl[dac0].modify(|_, w| w.runstdby().clear_bit());

//        *   Select dithering mode with DACCCTRLx.DITHER. Writing '1' to DITHER will enable dithering mode, writing a '0'
//            will disable it. Refer to 47.6.9.5. Dithering Mode for details.
        dac.dacctrl[dac0].modify(|_, w| w.dither().clear_bit());

//        *   Select the refresh period with the Refresh Period bit field in DACCCTRLx.REFRESH[3:0]. Writing any value
//            greater than '1' to the REFRESH bit field will enable and select the refresh mode.
//            Refer to 47.6.9.3. Conversion Refresh for details.


        /*  The DAC can only maintain its output within one LSB of the desired value for approximately 100μs
            refresh_1 means "Refresh every 30 us" which covers the 100μs mentioned above.                   */
        dac.dacctrl[dac0].modify(|_, w| w.refresh().refresh_1());

//        *   Select the output buffer current according to data rate (for low power application) with the Current Control bit
//            field DACCTRLx.CCTRL[1:0]. Refer to 47.6.9.2. Output Buffer Current Control for details.

        //  TBD
        dac.dacctrl[dac0].modify(|_, w| w.cctrl().cc12m());


//        *   Select standalone filter usage by writing to DACCTRLx.FEXT. Writing FEXT=1 selects a standalone filter,
//            FEXT=0 selects the filter integrated to the DAC. See also 47.6.9.6. Interpolation Mode for details./

/*          Bit 5 – FEXT External Filter Enable
                This bit controls the usage of the filter.
                    0       The filter is integrated to the DAC
                    1       The filter is used as standalone
            TBD: use "integrated" for now...                    */
        dac.dacctrl[dac0].modify(|_, w| w.fext().clear_bit());

//        *   Select the filter oversampling ratio by writing to DACCTRLx.OSR[2:0]. writing OSR=0 selects no oversampling;
//            writing any other value will enable interpolation of input data. See also 47.6.9.6. Interpolation Mode for details.
//            Once the DAC Controller is enabled, DACx requires a startup time before the first conversion can start. The DACx
//            Startup Ready bit in the Status register (STATUS.READYx) indicates that DACx is ready to convert a data when
//            STATUS.READYx=1.
//            Conversions started while STATUS.READYx=0 shall be discarded.
//            VOUTx is at tri-state level if DACx is not enabled.

/*            TBD:  For now use no oversampling/interpolation
                    0x0 OSR_1 1x OSR (no interpolation)      */
        dac.dacctrl[dac0].modify(|_, w| w.osr().osr_1());

        Self(dac)

    }

    pub fn enable_dac_controller(self) -> Self {
        /*  This enables the DAC controller which includes both
            DAC0 and DAC1.  */
        self.0.ctrla.modify(|_, w| w.enable().set_bit());
        while self.0.syncbusy.read().enable().bit_is_set() {};
        self
    }

    pub fn start_conversion(self, dac_i: usize) -> Self {
        //**************************************************************** */
        //  Todo: How to make function generic to handle both DAC0 and DAC1?
        //  HACK:
        let dac0: usize = 0;
        let dac1: usize = 1;
        //**************************************************************** */

//        self.0.data[dac_i].modify(|_, w| w.data());
        if 0 == dac_i{
            while self.0.syncbusy.read().data0().bit_is_set() {};
        } else {
            while self.0.syncbusy.read().data1().bit_is_set() {};
        }


        self.0.ctrla.modify(|_, w| w.enable().set_bit());
        while self.0.syncbusy.read().enable().bit_is_set() {};
        self
    }

}



