use std::{env, io, time::Duration};

use crsf::{Config, Packet, PacketReader};

fn main() {
    let path = env::args().nth(1).expect("no serial port supplied");
    let mut port = serialport::new(path, 115_200)
        .timeout(Duration::from_millis(20))
        .open()
        .expect("failed to open serial port");

    let mut buf = [0; 1024];
    let mut reader = PacketReader::new(Config::default());
    loop {
        match port.read(buf.as_mut_slice()) {
            Ok(n @ 1..) => {
                for result in reader.iter_packets(&buf[..n]) {
                    match result {
                        Ok(Packet::LinkStatistics(link_stats)) => {
                            println!("{:?}", link_stats);
                        }
                        Ok(Packet::RcChannelsPacked(rc_channels)) => {
                            println!("{:?}", rc_channels);
                        }
                        _ => {
                            eprintln!("Unknown packet");
                        }
                    }
                }
            }
            Ok(0) => {
                eprintln!("EOF");
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}
