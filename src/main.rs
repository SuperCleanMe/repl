use std::io::{Read, Stdin, Stdout, Stderr, stdin, stdout, stderr, Write, BufReader, BufRead, BufWriter};
use std::path::PathBuf;
use std::process::exit;

use orion::aead::*;
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use crate::structs::Character;
use std::net::TcpStream;

mod commands;
mod structs;

fn main() {
    println!("\x1b[33mStarting DBS REPL Version 0.1.0\x1b[0m");
    println!("\x1b[31mCopyright 2020 Thomas Bardsley\x1b[0m");

    let mut input_history = Vec::<String>::new();
    let mut stdin = stdin();
    let mut stdout = stdout();
    let mut stderr = stderr();
    let mut current_db = "None".to_string();
    // let mut location = std::env::current_dir().unwrap();
    let mut connection = TcpStream::connect("localhost:1234").unwrap();
    let mut r = BufReader::new(&mut connection);
    let mut is_key = true;
    let mut raw_key = Vec::<u8>::new();
    let mut key = SecretKey::default();
    loop {
        let mut byte: [u8; 1] = [0];
        r.read_exact(&mut byte);


        if is_key {
            let bt = byte.first().unwrap();
            match bt {
                10 => {
                    is_key = false;
                    // println!("key: {:?}", key);
                    let sec = SecretKey::from_slice(raw_key.as_slice());
                    if sec.is_ok() {
                        println!("Key created correctly");
                        key = sec.unwrap();
                        println!("Key match: {}", key == raw_key.as_slice());
                    } else {
                        println!("{}", sec.unwrap_err());
                    }
                }
                _ => {
                    raw_key.push(bt.clone());
                }
            }
        } else {
            println!("Sending response");
            let mut writer = BufWriter::new(&mut connection);
            let payload = seal(&key, b"Hello World").unwrap();
            writer.write_all(payload.as_slice());
            writer.flush();
            exit(1);
        }


        // let input = read_input(&current_db, &mut stdin, &mut stdout, &mut stderr, &input_history);
        //
        // connection.write_all(format!("{}\n", input).as_bytes());
        // connection.flush();

        // input_history.push(input.clone());
        // current_db = commands::parse(input, &current_db);
    }
}

fn read_input(cdb: &String, sin: &mut Stdin, out: &mut Stdout, err: &mut Stderr, history: &Vec<String>) -> String {
    print!("\x1b[32m{}\x1b[0m > ", cdb);
    out.flush();
    let mut buf = Vec::<u8>::new();
    let mut input = String::new();
    let mut byte: [u8; 1] = [0];
    let filter = b"\n";
    let chars = Vec::<char>::new();
    while byte != structs::NEWLINE {
        let _ = sin.read(&mut byte);
        buf.push(byte[0]);
    }

    input = String::from_utf8(buf).unwrap();

    input.pop();
    input
}
