use std::{fs, io, thread::sleep, time::Duration};

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

fn main() -> eyre::Result<()> {
    let args = Args::parse();

    let imgs =
        compress::decompress_imgs(fs::File::open(args.input)?).collect::<Result<Vec<_>, _>>()?;
    let mut device = serialport::new(args.dev_path, 115_200).open()?;
    device.set_timeout(Duration::from_millis(10))?;

    for ele in imgs {
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
    }

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
