use std::io::{Read, Write, BufReader, BufWriter};
use std::net::TcpStream;
use std::process::exit;

use orion::aead::*;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream
        }
    }

    pub fn read_ley(&mut self) -> SecretKey {
        let mut r = BufReader::new(&mut self.stream);
        let mut bytes: [u8; 72] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        let res = r.read_exact(&mut bytes);
        let mut key = SecretKey::default();

        if res.is_ok() {
            let sec = SecretKey::from_slice(&bytes);
            if sec.is_ok() {
                println!("Key created correctly");
                key = sec.unwrap();
            } else {
                println!("{}", sec.unwrap_err());
            }
        } else {
            println!("\x1b[31mAn error occured during key transfer. Exiting...\x1b[0m");
            exit(1);
            // println!("READ ERR: {}", res.unwrap_err());
        }

        return key;
    }

    pub fn write_all(&mut self, key: &SecretKey, content: &str) {
        let mut writer = BufWriter::new(&mut self.stream);
        let res = seal(key, content.as_bytes());
        if res.is_ok() {
            writer.write_all(res.unwrap().as_slice());
            writer.flush();
        } else {
            println!("Encountered an encryption error:\n{}", res.unwrap_err());
        }
    }
}