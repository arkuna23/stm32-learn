use crate::consts;

use super::consts::SSD1306Cmd;
use embedded_hal::blocking::i2c::Write;

pub struct Oled<const M: usize, I: Write> {
    i2c: I,
}

impl<const H: usize, I: Write> Oled<H, I> {
    pub fn new(i2c: I) -> Self {
        Oled { i2c }
    }

    pub fn write<const N: usize, const F: u8>(&mut self, data: &[u8]) -> Result<(), I::Error> {
        let mut buffer = [0; N];
        buffer[0] = F;

        data.chunks(N - 1).try_for_each(|c| {
            let len = c.len();
            buffer[1..=len].copy_from_slice(c);
            self.i2c.write(consts::SSD1306_ADDR, &buffer)
        })
    }

    #[inline(always)]
    pub fn send_one_byte_cmd(&mut self, cmd: u8) -> Result<(), I::Error> {
        self.i2c.write(consts::SSD1306_ADDR, &[0x80, cmd])
    }

    #[inline(always)]
    pub fn send_one_byte_cmds(&mut self, cmds: &[u8]) -> Result<(), I::Error> {
        for ele in cmds {
            self.send_one_byte_cmd(*ele)?;
        }
        Ok(())
    }

    #[inline(always)]
    pub fn send_cmd(&mut self, cmd: &[u8]) -> Result<(), I::Error> {
        self.write::<{ consts::CMD_BUFFER_SIZE + 1 }, 0x00>(cmd)
    }

    #[inline(always)]
    pub fn send_data(&mut self, data: &[u8]) -> Result<(), I::Error> {
        self.write::<{ consts::DATA_BUFFER_SIZE + 1 }, 0x40>(data)
    }

    #[inline(always)]
    pub fn send_data_custom<const N: usize>(&mut self, data: &[u8]) -> Result<(), I::Error> {
        self.write::<N, 0x40>(data)
    }

    pub fn set_display_addr(&mut self, col: (u8, u8), page: (u8, u8)) -> Result<(), I::Error> {
        self.send_cmd(&[SSD1306Cmd::SET_COLUMN_ADDR, col.0, col.1])?;
        self.send_cmd(&[SSD1306Cmd::SET_PAGE_ADDR, page.0, page.1])
    }

    pub fn init(&mut self) -> Result<(), I::Error> {
        self.send_one_byte_cmd(SSD1306Cmd::DISPLAY_OFF)?;
        self.send_cmd(&[SSD1306Cmd::SET_DISPLAY_CLOCK_DIV, 0x80])?;
        self.send_one_byte_cmd(SSD1306Cmd::SET_MULTIPLEX | (H as u8 - 1))?;
        self.send_cmd(&[SSD1306Cmd::SET_DISPLAY_OFFSET, 0x00])?;
        self.send_cmd(&[SSD1306Cmd::SET_CHARGE_PUMP, SSD1306Cmd::CHARGE_PUMP_ENABLE])?;
        self.send_one_byte_cmds(&[
            SSD1306Cmd::MEMORY_MODE,
            SSD1306Cmd::SET_DISPLAY_START_LINE,
            SSD1306Cmd::SEG_REMAP | 0x1,
            SSD1306Cmd::COM_SCAN_DEC,
        ])?;
        self.send_cmd(&[SSD1306Cmd::SET_COM_PINS, 0x12])?;
        self.send_cmd(&[SSD1306Cmd::SET_CONTRAST, 0xCF])?;
        self.send_cmd(&[SSD1306Cmd::SET_PRECHARGE, 0xF1])?;
        self.send_cmd(&[SSD1306Cmd::SET_VCOM_DETECT, 0x40])?;
        self.send_one_byte_cmds(&[
            SSD1306Cmd::DISPLAY_ALL_ON_RESUME,
            SSD1306Cmd::NORMAL_DISPLAY,
            SSD1306Cmd::DISPLAY_ON,
        ])
    }

    pub fn clear(&mut self) -> Result<(), I::Error> {
        for _ in 0..H / 8 {
            self.send_data(&[0xff; 128])?;
        }
        Ok(())
    }
}
