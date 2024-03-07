use std::{env, io, time::Duration};

use crsf::{Packet, PacketParser};

fn main() {
    let path = env::args().nth(1).expect("no serial port supplied");
    let mut port = serialport::new(path, 115_200)
        .timeout(Duration::from_millis(20))
        .open()
        .expect("failed to open serial port");

    let mut buf = [0; 1024];
    let mut parser = PacketParser::<1024>::new();
    loop {
        match port.read(buf.as_mut_slice()) {
            Ok(n) => {
                if n > 0 {
                    parser.push_bytes(&buf[..n]);
                    while let Some(Ok(packet)) = parser.next_packet() {
                        match packet {
                            Packet::LinkStatistics(link_statistics) => {
                                println!("{:?}", link_statistics);
                            }
                            Packet::RcChannels(channels) => {
                                println!("{:?}", channels);
                            }
                        }
                    }
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}
