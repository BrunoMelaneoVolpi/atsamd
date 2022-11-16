

//#![no_std]
//#![no_main]



pub trait Default {
    fn default(&self);
}
pub trait BuildCommand {
    fn build_command(&self);
}

pub enum CmdType {
    READ = 0b00,
    WRITE = 0b11,
}


pub enum MemAdd {
    Dac0OutputAdd   = 0x00,
    Dac1OutputAdd   = 0x01,
    VrefAdd         = 0x08,
    PowerDownAdd    = 0x09,
    GainAdd         = 0x0A,
}


//#[bitfield]
//#[repr(u8)]
//#[derive(Clone, Copy)]
pub struct CommandByte {
    pub mem_add:    MemAdd,     //  Memory address
    pub cmd_type:   CmdType,    //  Read, Write
    pub cmd_err:    bool,       //  CmdType,
}


//  ---------------------------------------------------------------------
pub struct DataWord0x0xDacOutput{
/*  3FFh = Full-Scale output value
    1FFh = Mid-Scale output value
    000h = Zero-Scale output value   */
    output: u16
}


pub struct Cmd0x00DacOutput{
    command_byte:   CommandByte,
    data_word:      DataWord0x0xDacOutput
}

pub struct Cmd0x01DacOutput{
    command_byte:   CommandByte,
    data_word:      DataWord0x0xDacOutput
}
//  ---------------------------------------------------------------------


pub enum VrefSource {
    VrefBuffered    = 0b11,     //  11 = VREF pin (Buffered); VREF buffer enabled
    VrefUnbuffered  = 0b10,     //  10 = VREF pin (Unbuffered); VREF buffer disabled
    InternalBandGap = 0b01,     //  01 = Internal Band Gap (1.22V typical); VREF buffer enabled VREF voltage driven when powered-down
    VddUnbuffered   = 0b00,     //  00 = VDD (Unbuffered); VREF buffer disabled. Use this state with Power-Down bits for lowest current.
}



pub struct DataWord0x08Vref{
    dac_0_v_ref_src : VrefSource,
    dac_1_v_ref_src : VrefSource,
}

pub struct Cmd0x08Vref{
    command_byte:   CommandByte,
    data_word:      DataWord0x08Vref
}

//  ---------------------------------------------------------------------


pub enum Power {
    NormalOperation         = 0b00,     //  00 =Normal Operation (Not powered-down)
    PoweredDown1kToGround   = 0b01,     //  01 =Powered Down - VOUT is loaded with a 1 k resistor to ground.
    PoweredDown100kToGround = 0b10,     //  10 =Powered Down - VOUT is loaded with a 100 k resistor to ground.
    PoweredDownOpenCircuit  = 0b11,     //  11 =Powered Down - VOUT is open circuit.
}


pub struct DataWord0x09PowerDown{
    dac_0_pwer : Power,
    dac_1_pwer : Power,
}

pub struct Cmd0x09PowerDown{
    command_byte:   CommandByte,
    data_word:      DataWord0x09PowerDown
}
//  ---------------------------------------------------------------------
pub struct DataWord0x0AGain{}

pub struct Cmd0x0AGain{
    command_byte:   CommandByte,
    data_word:      DataWord0x0AGain
}


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
