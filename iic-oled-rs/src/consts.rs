pub const SSD1306_ADDR: u8 = 0x3C;

pub struct SSD1306Cmd;

impl SSD1306Cmd {
    pub const SET_CONTRAST: u8 = 0x81;
    pub const DISPLAY_ALL_ON_RESUME: u8 = 0xA4;
    pub const DISPLAY_ALL_ON: u8 = 0xA5;
    pub const NORMAL_DISPLAY: u8 = 0xA6;
    pub const INVERT_DISPLAY: u8 = 0xA7;
    pub const DISPLAY_OFF: u8 = 0xAE;
    pub const DISPLAY_ON: u8 = 0xAF;
    pub const SET_DISPLAY_OFFSET: u8 = 0xD3;
    pub const SET_COM_PINS: u8 = 0xDA;
    pub const SET_VCOM_DETECT: u8 = 0xDB;
    pub const SET_DISPLAY_CLOCK_DIV: u8 = 0xD5;
    pub const SET_PRECHARGE: u8 = 0xD9;
    pub const SET_MULTIPLEX: u8 = 0xA8;
    pub const SET_CHARGE_PUMP: u8 = 0x8D;
    pub const CHARGE_PUMP_ENABLE: u8 = 0x14;
    pub const CHARGE_PUMP_DISABLE: u8 = 0x10;
    pub const MEMORY_MODE: u8 = 0x20;
    pub const SET_COLUMN_ADDR: u8 = 0x21;
    pub const SET_PAGE_ADDR: u8 = 0x22;
    pub const COM_SCAN_INC: u8 = 0xC0;
    pub const COM_SCAN_DEC: u8 = 0xC8;
    pub const SEG_REMAP: u8 = 0xA0;
    pub const SET_COM_PINS_ALT: u8 = 0xDA;
    pub const SET_DISPLAY_START_LINE: u8 = 0x40;
}

pub const CMD_BUFFER_SIZE: usize = 4;
pub const DATA_BUFFER_SIZE: usize = 128;
