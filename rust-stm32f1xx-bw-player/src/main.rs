#![no_std]
#![no_main]
#![deny(unsafe_code)]

use bw_img_comm::{Signal, FULL_DATA_BYTE};
use cortex_m::asm::delay;
use defmt::println;
use defmt_rtt as _;
use fugit::RateExtU32;
use iic_oled_rs::Oled;
use panic_probe as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::afio::AfioExt;
use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::i2c::{BlockingI2c, DutyCycle, Mode};
use stm32f1xx_hal::pac;
use stm32f1xx_hal::prelude::_stm32_hal_flash_FlashExt;
use stm32f1xx_hal::rcc::RccExt;
use stm32f1xx_hal::usb::Peripheral;
use usb_device::bus::UsbBus;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usb_device::UsbError;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    // 获取对外设的访问对象
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // 获得原始flash和rcc设备的所有权，并将它们转换为相应的HAL结构
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // 冻结系统中所有时钟的配置，并将冻结的频率存储在时钟中
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(72.MHz())
        .pclk1(36.MHz())
        .pclk2(72.MHz())
        .freeze(&mut flash.acr);

    println!("init oled display");
    let mut afio = dp.AFIO.constrain();
    let mut gpiob = dp.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);
    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400u32.kHz(),
            duty_cycle: DutyCycle::Ratio16to9,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );
    let mut oled = Oled::<64, _>::new(i2c);
    oled.init().unwrap();
    oled.vertical_mem_mode().unwrap();
    oled.clear().unwrap();

    println!("init usb serial");
    let mut gpioa = dp.GPIOA.split();
    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);
    usb_dp.set_low();
    delay(clocks.sysclk().raw() / 100);
    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
    };

    let usb_bus = stm32f1xx_hal::usb::UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .device_class(USB_CLASS_CDC)
        .product("Serial port")
        .build();

    let mut opcode = [0u8; 1];
    let mut buffer = [0u8; { 128 * 8 }];

    println!("start main loop");
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        let Ok(count) = serial.read(&mut opcode) else {
            continue;
        };
        if count == 0 {
            continue;
        }

        // 从串口读取数据
        let signal = match opcode[0] {
            FULL_DATA_BYTE => {
                let mut offset = 0;
                while offset < buffer.len() {
                    match serial.read(&mut buffer[offset..]) {
                        Ok(count) => {
                            offset += count;
                        }
                        Err(UsbError::WouldBlock) => {
                            continue;
                        }
                        Err(e) => {
                            panic!("read error: {:?}", e);
                        }
                    }
                }
                Signal::FullData(&buffer)
            }
            _ => Signal::new(opcode[0], None).unwrap(),
        };

        match signal {
            Signal::FullData(data) => {
                oled.send_data(data).unwrap();
                serial_write(&mut serial, Signal::CommACK)
            }
            _ => continue,
        }
    }
}

fn serial_write<B: UsbBus>(serial: &mut SerialPort<B>, signal: Signal) {
    let (first, data) = signal.to_bytes();

    match data {
        Some(data) => {
            serial.write(&[first]).unwrap();
            let mut offset = 0;
            while offset < data.len() {
                let count = serial.write(&data[offset..]).unwrap();
                offset += count;
            }
        }
        None => {
            serial.write(&[first]).unwrap();
        }
    }
}
