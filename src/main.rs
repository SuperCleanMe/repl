use std::io::{Read, Stdin, Stdout, Stderr, stdin, stdout, stderr, Write, BufReader, BufRead, BufWriter};
use std::path::PathBuf;
use std::process::exit;
use std::net::TcpStream;

use orion::aead::*;
use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use crate::structs::*;

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

    let con_res = TcpStream::connect("localhost:1234");
    if con_res.is_ok() {
        let mut connection = Connection::new(con_res.unwrap());
        let mut key = connection.read_ley();
        connection.write_all(&key, "Hello, world\n");
        // loop {
        //         println!("Sending response");
        // }
    } else {
        println!("Connection attempt unsuccessful");
        println!("{}", con_res.unwrap_err());
    }
}

// fn read_input(cdb: &String, sin: &mut Stdin, out: &mut Stdout, err: &mut Stderr, history: &Vec<String>) -> String {
//     print!("\x1b[32m{}\x1b[0m > ", cdb);
//     out.flush();
//     let mut buf = Vec::<u8>::new();
//     let mut input = String::new();
//     let mut byte: [u8; 1] = [0];
//     let filter = b"\n";
//     let chars = Vec::<char>::new();
//     while byte != structs::NEWLINE {
//         let _ = sin.read(&mut byte);
//         buf.push(byte[0]);
//     }

//     input = String::from_utf8(buf).unwrap();

//     input.pop();
//     input
// }
