use std::{io::{self, Read, StdoutLock, Write}, sync::Arc};

use serde::{Deserialize, Serialize};

// so this currently only does one thing and that is :
// prints the files in the current directory to the stdout
//
//
// todos:
// 1. add commands and a way to communicate to the lua module using some kind of rpc
// 2. listing files, fuzzy file search , fuzzy search in current directory
//

// read msgpack
// decode array
// handle method
// write response
//


fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let stdout = io::stdout();
    let mut output = stdout.lock();

    loop {
        let msg = read_value(&mut input).unwrap();

        if let Value::Array(arr) = msg {
            handle_message(arr, &mut output).unwrap();
        }
    }
    // for entry in walkdir::WalkDir::new(".") {
    //     let entry = entry.unwrap();
    //     if entry.file_type().is_file() {
    //         println!("{}", entry.path().display());
    //     }
    // }
    // io::stdout().flush().unwrap();
}

fn handle_message<W: Write>(msg: Vec<Value>, out: &mut W) -> io::Result<()> {
    if msg.is_empty() {
        return Ok(());
    }

    match msg[0] {
        Value::Int(0) => handle_request(msg, out),
        Value::Int(2) => {
            // notification
            println!("notification received");
            Ok(())
        }
        _ => Ok(())
    }
}

fn handle_request<W:Write> (msg: Vec<Value> , out: &mut W) -> io::Result<()>{
    let msgid = match msg.get(1) {
        Some(Value::Int(i)) => *i as u8,
        _ => return Ok(()),
    };

    let method = match msg.get(2) {
        Some(Value::Str(s)) => s,
        _ => return Ok(()),
    };

    match method.as_str() {
        "ping" => send_response(out, msgid, Some("pong")),
        _ => send_response(out, msgid, None),
    }
}

fn send_response<W: Write>(out: &mut W, msgid: u8, result: Option<&str>)-> io::Result<()>{
    write_array(out, 4)?;
    write_int(out, 1)?;
    write_int(out, msgid)?;
    write_nil(out)?;

    match result {
        Some(r) => write_str(out, r)?,
        None => write_nil(out)?,
    }

    out.flush()
}


#[derive(Debug)]
enum Value{
    Int(i64),
    Str(String),
    Array(Vec<Value>),
    Nil,
}


fn read_value<R: Read>(r: &mut R) -> io::Result<Value>{
    let mut b = [0u8];
    r.read_exact(&mut b)?;
    match b[0] {
        0x00..=0x7f => Ok(Value::Int(b[0] as i64)),

        0x90..=0x9f => {
            let len = (b[0] & 0x0f) as usize;
            let mut arr = Vec::with_capacity(len);
            for _ in 0..len{
                arr.push(read_value(r)?);
            }
            Ok(Value::Array(arr))

        },

        0xa0..=0xbf => {
            let len = (b[0] & 0x1f) as usize;
            let mut buf = vec![0; len];
            r.read_exact(&mut buf)?;
            Ok(Value::Str(String::from_utf8(buf).unwrap()))

        },

        0xc0 => Ok(Value::Nil),

        _=> panic!("unsupported")


    }
}


fn write_array<W: Write>(w: &mut W, len: usize) -> io::Result<()> {
    w.write_all(&[0x90 | len as u8])
}

fn write_str<W: Write>(w: &mut W, s: &str) -> io::Result<()> {
    w.write_all(&[0xa0 | s.len() as u8])?;
    w.write_all(s.as_bytes())
}

fn write_int<W: Write>(w: &mut W, n: u8) -> io::Result<()> {
    w.write_all(&[n])
}

fn write_nil<W: Write>(w: &mut W) -> io::Result<()> {
    w.write_all(&[0xc0])
}
