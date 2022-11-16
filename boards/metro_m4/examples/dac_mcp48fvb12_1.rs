//#![no_std]
//#![no_main]

use rtt_target::{rprint, rprintln, rtt_init_print};

//  ======  Functions    ===========================
fn new_cmd_byte(mem_add: MemAdd, cmd_type: CmdType) -> CommandByte{
    CommandByte{
        mem_add:    mem_add,        //  Memory address
        cmd_type:   cmd_type,       //  Read, Write
        cmd_err:    false,          //  CmdType,
    }
}

fn new_cmd_stream(cmd: & CommandByte) -> u8{

    let command_byte =  ((cmd.mem_add as u8) << 3) |
                            ((cmd.cmd_type as u8) << 1) |
                            ((cmd.cmd_err as u8));

    command_byte
}

//  ======  Traits    ==============================
pub trait Default {
    fn default(&self) -> Self;
}


//pub const DAC_COMMAND_SIZE: usize = 3;
//pub type CommandStream = [u8; DAC_COMMAND_SIZE];

#[derive(Default)]
#[derive(Debug)]
#[repr(C)]
pub struct CommandStream{
    command_byte:   u8,
    data_word:      u16
}

pub trait BuildCommand {
    fn build_command(&self) -> CommandStream;
}

pub trait SetCommandType{
    fn set_cmd_type(& mut self, cmd_type: CmdType) -> ();
}

//  ======  Type definitions    ====================
#[derive(Debug, PartialEq)]
pub enum DacId {
    Dac0 = 0,
    Dac1 = 1,
}


#[derive(Debug)]
#[repr(u8)]
pub enum CmdType {
    READ = 0b00,
    WRITE = 0b11,
}

#[derive(Debug)]
#[repr(u8)]
pub enum MemAdd {
    Dac0OutputAddress   = 0x00,
    Dac1OutputAddress   = 0x01,
    VrefAddress         = 0x08,
    PowerDownAddress    = 0x09,
    GainAddress         = 0x0A,
}

//#[bitfield]
//#[repr(u8)]
//#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct CommandByte {
    pub mem_add:    MemAdd,     //  Memory address
    pub cmd_type:   CmdType,    //  Read, Write
    pub cmd_err:    bool,       //  CmdType,
}

//  ---------------------------------------------------------------------
#[derive(Debug)]
pub struct DataWord0x0xDacOutput{
/*  3FFh = Full-Scale output value
    1FFh = Mid-Scale output value
    000h = Zero-Scale output value   */
    output: u16
}

#[derive(Debug)]
pub struct CmdDacOutput{
    command_byte:   CommandByte,
    data_word:      DataWord0x0xDacOutput
}

impl CmdDacOutput{
    pub fn new(dac_i: DacId) -> CmdDacOutput{
        let address =
            match dac_i{
                DacId::Dac0 => MemAdd::Dac0OutputAddress,
                DacId::Dac1 => MemAdd::Dac1OutputAddress,
            };

        CmdDacOutput{
            command_byte:   new_cmd_byte(   address,
                                            CmdType::WRITE),
            data_word:      DataWord0x0xDacOutput{output: 0}
        }
    }

    pub fn set_output(& mut self, value: u16) -> (){
        self.data_word.output = value;
    }
}



impl SetCommandType for CmdDacOutput{
    fn set_cmd_type(& mut self, cmd_type: CmdType) -> (){
        self.command_byte.cmd_type = cmd_type
    }
}


impl BuildCommand for CmdDacOutput{
    fn build_command(&self) -> CommandStream{
        let stream = CommandStream{
            command_byte:   new_cmd_stream( & self.command_byte),
            data_word:      self.data_word.output
        };

        //rprintln!("   stream :: CmdDacOutput :: {:?}", stream);

        //  return the built command
        stream
    }
}



//  ---------------------------------------------------------------------
#[derive(Debug)]
pub enum VrefSource {
    VrefBuffered    = 0b11,     //  11 = VREF pin (Buffered); VREF buffer enabled
    VrefUnbuffered  = 0b10,     //  10 = VREF pin (Unbuffered); VREF buffer disabled
    InternalBandGap = 0b01,     //  01 = Internal Band Gap (1.22V typical); VREF buffer enabled VREF voltage driven when powered-down
    VddUnbuffered   = 0b00,     //  00 = VDD (Unbuffered); VREF buffer disabled. Use this state with Power-Down bits for lowest current.
}

#[derive(Debug)]
pub struct DataWord0x08Vref{
    dac_0_v_ref_src : VrefSource,
    dac_1_v_ref_src : VrefSource,
}

#[derive(Debug)]
pub struct Cmd0x08Vref{
    command_byte:   CommandByte,
    data_word:      DataWord0x08Vref
}

impl Cmd0x08Vref{
    pub fn new() -> Cmd0x08Vref{
        Cmd0x08Vref{
            command_byte:   new_cmd_byte(   MemAdd::VrefAddress,
                                            CmdType::WRITE),
            data_word:      DataWord0x08Vref{   dac_0_v_ref_src: VrefSource::VrefBuffered,
                                                dac_1_v_ref_src: VrefSource::VrefBuffered
            }
        }
    }

    pub fn set_vref(& mut self,
                    dac_0_vref: VrefSource,
                    dac_1_vref: VrefSource) -> (){
        self.data_word.dac_0_v_ref_src = dac_0_vref;
        self.data_word.dac_1_v_ref_src = dac_1_vref;
    }
}


impl SetCommandType for Cmd0x08Vref{
    fn set_cmd_type(& mut self, cmd_type: CmdType) -> (){
        self.command_byte.cmd_type = cmd_type
    }
}


impl BuildCommand for Cmd0x08Vref{
    fn build_command(&self) -> CommandStream{

        const DAC1_VREF_OFFSET  :u8 = 2;
        const DAC0_VREF_OFFSET  :u8 = 0;

        let stream = CommandStream{
            command_byte:   new_cmd_stream( & self.command_byte),
            data_word:      ((self.data_word.dac_1_v_ref_src as u16) << DAC1_VREF_OFFSET) |
                            ((self.data_word.dac_0_v_ref_src as u16) << DAC0_VREF_OFFSET)
        };

        //rprintln!("   stream :: Cmd0x08Vref :: {:?}", stream);

        //  return the built command
        stream
    }
}



//  ---------------------------------------------------------------------
#[derive(Debug)]
pub enum Power {
    NormalOperation         = 0b00,     //  00 =Normal Operation (Not powered-down)
    PoweredDown1kToGround   = 0b01,     //  01 =Powered Down - VOUT is loaded with a 1 k resistor to ground.
    PoweredDown100kToGround = 0b10,     //  10 =Powered Down - VOUT is loaded with a 100 k resistor to ground.
    PoweredDownOpenCircuit  = 0b11,     //  11 =Powered Down - VOUT is open circuit.
}

#[derive(Debug)]
pub struct DataWord0x09PowerDown{
    dac_0_power : Power,
    dac_1_power : Power,
}

#[derive(Debug)]
pub struct Cmd0x09PowerDown{
    command_byte:   CommandByte,
    data_word:      DataWord0x09PowerDown
}


impl Cmd0x09PowerDown{
    pub fn new() -> Cmd0x09PowerDown{
        Cmd0x09PowerDown{
            command_byte:   new_cmd_byte(   MemAdd::PowerDownAddress,
                                            CmdType::WRITE),
            data_word:      DataWord0x09PowerDown{  dac_0_power: Power::NormalOperation,
                                                    dac_1_power: Power::NormalOperation
            }
        }
    }

    pub fn set_power_down(& mut self,
                            dac_0_power: Power,
                            dac_1_power: Power) -> (){
        self.data_word.dac_0_power = dac_0_power;
        self.data_word.dac_1_power = dac_1_power;
    }
}


impl BuildCommand for Cmd0x09PowerDown{
    fn build_command(&self) -> CommandStream{

        const DAC1_POWER_OFFSET  :u8 = 2;
        const DAC0_POWER_OFFSET  :u8 = 0;

        let stream = CommandStream{
            command_byte:   new_cmd_stream( & self.command_byte),
            data_word:      ((self.data_word.dac_1_power as u16) << DAC1_POWER_OFFSET) |
                            ((self.data_word.dac_0_power as u16) << DAC0_POWER_OFFSET)
        };

        //rprintln!("   stream :: Cmd0x08Vref :: {:?}", stream);

        //  return the built command
        stream
    }
}

//  ---------------------------------------------------------------------

#[derive(Debug)]
pub enum Por {
    PorHasNotOccurred   = 0b0,  //  0 = A POR (BOR) event has not occurred since the last read of this register.
    PorHasOccurred      = 0b1,  //  1 = A POR (BOR) event occurred since the last read of this register. Reading this register clears this bit.
}

#[derive(Debug)]
pub enum Gain {
    Gainx1  = 0b0,              //  0 = 1 x Gain
    Gainx2  = 0b1,              //  1 = 2 x Gain
}

#[derive(Debug)]
pub struct DataWord0x0AGain{
    dac_0_gain  : Gain,
    dac_1_gain  : Gain,
    por         : Por
}

#[derive(Debug)]
pub struct Cmd0x0AGain{
    command_byte:   CommandByte,
    data_word:      DataWord0x0AGain
}


impl Cmd0x0AGain{
    pub fn new() -> Cmd0x0AGain{
        Cmd0x0AGain{
            command_byte:   new_cmd_byte(   MemAdd::GainAddress,
                                            CmdType::WRITE),
            data_word:      DataWord0x0AGain{   dac_0_gain: Gain::Gainx1,
                                                dac_1_gain: Gain::Gainx1,
                                                por       : Por::PorHasNotOccurred
            }
        }
    }

    pub fn set_gain(& mut self,
                    dac_0_gain: Gain,
                    dac_1_gain: Gain) -> (){
        self.data_word.dac_0_gain = dac_0_gain;
        self.data_word.dac_1_gain = dac_1_gain;
    }
}

impl BuildCommand for Cmd0x0AGain{
    fn build_command(&self) -> CommandStream{

        const DAC1_GAIN_OFFSET  :u8 = 9;
        const DAC0_GAIN_OFFSET  :u8 = 8;
        const POR_OFFSET        :u8 = 7;

        let stream = CommandStream{
            command_byte:   new_cmd_stream( & self.command_byte),
            data_word:      ((self.data_word.dac_1_gain as u16) << DAC1_GAIN_OFFSET) |
                            ((self.data_word.dac_0_gain as u16) << DAC0_GAIN_OFFSET) |
                            ((self.data_word.por as u16) << POR_OFFSET)
        };

        //rprintln!("   stream :: Cmd0x08Vref :: {:?}", stream);

        //  return the built command
        stream
    }
}

//  ======  Implementations    =====================

//  ======  Functions    ===========================


//0x00DacOutput
//0x01DacOutput
//0x08Vref
//0x09PowerDown
//0x0AGain


//#[repr(C)]
//pub struct Command0x00DacOutput{
//
//    fields x
//    fields y
//
//}