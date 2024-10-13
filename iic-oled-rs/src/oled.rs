use crate::consts;

use super::consts::SSD1306Cmd;
use embedded_hal::blocking::i2c::Write;

pub struct Oled<const M: usize, I: Write> {
    i2c: I,
}

impl<const M: usize, I: Write> Oled<M, I> {
    pub fn new(i2c: I) -> Self {
        Oled { i2c }
    }

    fn buf_write<const N: usize, const F: u8>(&mut self, cmds: &[u8]) -> Result<(), I::Error> {
        let mut buffer = [0; N];
        buffer[0] = F;

        let mut current = 0;
        for cmd in cmds {
            buffer[current + 1] = *cmd;
            current += 1;
            if current == N - 1 {
                self.i2c.write(consts::SSD1306_ADDR, &buffer)?;
                current = 0;
            }
        }

        if current > 0 {
            self.i2c.write(consts::SSD1306_ADDR, &buffer[..current + 1])
        } else {
            Ok(())
        }
    }

    pub fn send_cmd_single(&mut self, cmd: u8) -> Result<(), I::Error> {
        self.i2c.write(consts::SSD1306_ADDR, &[0x80, cmd])
    }

    pub fn send_data_single(&mut self, data: u8) -> Result<(), I::Error> {
        self.i2c.write(consts::SSD1306_ADDR, &[0xC0, data])
    }

    #[inline(always)]
    pub fn send_cmds(&mut self, cmds: &[u8]) -> Result<(), I::Error> {
        self.buf_write::<{ consts::CMD_BUFFER_SIZE + 1 }, 0x00>(cmds)
    }

    #[inline(always)]
    pub fn send_data(&mut self, data: &[u8]) -> Result<(), I::Error> {
        self.buf_write::<{ consts::DATA_BUFFER_SIZE + 1 }, 0x40>(data)
    }

    pub fn set_display_addr(&mut self, col: (u8, u8), page: (u8, u8)) -> Result<(), I::Error> {
        self.send_cmds(&[
            SSD1306Cmd::SET_COLUMN_ADDR,
            SSD1306Cmd::SET_PAGE_ADDR,
        ])
    }

    pub fn init(&mut self) -> Result<(), I::Error> {
        self.send_cmds(&[
            SSD1306Cmd::DISPLAY_OFF,
            SSD1306Cmd::SET_DISPLAY_CLOCK_DIV,
            0x80,
            SSD1306Cmd::SET_MULTIPLEX,
            M as u8 - 1,
            SSD1306Cmd::SET_DISPLAY_OFFSET,
            0x00,
            SSD1306Cmd::SET_DISPLAY_START_LINE,
            SSD1306Cmd::SET_CHARGE_PUMP,
            SSD1306Cmd::CHARGE_PUMP_ENABLE,
            SSD1306Cmd::SEG_REMAP | 0x1,
            SSD1306Cmd::COM_SCAN_DEC,
            SSD1306Cmd::SET_COM_PINS,
            0x12,
            SSD1306Cmd::SET_CONTRAST,
            0xCF,
            SSD1306Cmd::SET_PRECHARGE,
            0xF1,
            SSD1306Cmd::SET_VCOM_DETECT,
            0x40,
            SSD1306Cmd::DISPLAY_ALL_ON_RESUME,
            SSD1306Cmd::NORMAL_DISPLAY,
            SSD1306Cmd::DISPLAY_ON,
        ])
    }

    pub fn clear(&mut self) -> Result<(), I::Error> {
        self.set_display_addr((0, 128), (0, M as u8))?;
        for _ in 0..M {
            self.send_data(&[0x00; 128])?;
        }
        Ok(())
    }
}
