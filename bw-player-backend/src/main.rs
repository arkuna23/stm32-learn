use std::{fs, io::{self, Write}, thread::sleep, time::Duration};

use bw_img::{file::compress, iter_direction, IterOutput};
use bw_img_comm::{COMM_ACK_BYTE, FULL_DATA_BYTE};
use clap::Parser;
use eyre::Context;

#[derive(clap::Parser)]
struct Args {
    #[clap(short, long)]
    input: String,
    #[clap(short, long, default_value = "/dev/ttyACM0")]
    dev_path: String,
}

const FRAME_RATE: u32 = 30;

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let imgs =
        compress::decompress_imgs(fs::File::open(args.input)?).collect::<Result<Vec<_>, _>>()?;
    let mut device = serialport::new(args.dev_path, 115_200).open()?;
    device.set_timeout(Duration::from_millis(5))?;

    let mut frame_rate = 0;
    let mut current_time = std::time::Instant::now();
    println!("Start sending images");
    let duration = Duration::from_secs(1) / FRAME_RATE;
    for ele in imgs {
        let started_ins = std::time::Instant::now();
        let img: Vec<_> = ele
            .iterator(iter_direction::VerticalRev)
            .filter_map(|r| {
                if let IterOutput::Byte { byte, len: _ } = r {
                    Some(byte)
                } else {
                    None
                }
            })
            .collect();
        device.write_all(&[FULL_DATA_BYTE])?;
        sleep(Duration::from_millis(1));

        device.write_all(&img[..])?;
        read_ack(&mut device).wrap_err_with(|| "read err")?;
        frame_rate += 1;

        if started_ins.elapsed() < duration {
            sleep(duration - started_ins.elapsed());
        }
        if current_time.elapsed().as_secs() >= 1 {
            print!("\r                        ");
            print!("\rFrame rate: {}", frame_rate);
            std::io::stdout().flush().unwrap();
            frame_rate = 0;
            current_time = std::time::Instant::now();
        }
    }
    println!();

    Ok(())
}

fn read_ack(device: &mut Box<dyn serialport::SerialPort>) -> eyre::Result<()> {
    let mut opcode = [0u8; 1];
    while let Err(e) = device.read(&mut opcode) {
        if e.kind() != io::ErrorKind::TimedOut {
            eyre::bail!("Error reading from device: {:?}", e);
        }
    }
    if COMM_ACK_BYTE != opcode[0] {
        eyre::bail!("Invalid signal received");
    }
    Ok(())
}
