// SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::{
    Read,
    Write,
    stdout,
    stdin

};

use std::env;
use std::fs;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const TAPE_LEN: usize = 30_000;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
	loop {
	    print!(">>> ");
	    let _ = stdout().flush();
	    let mut input = String::new();
	    match stdin().read_line(&mut input) {
		Ok(n) => {
		    interpreter(&input);
		}
		Err(error) => println!("error: {error}"),
	    }
	}
    }

    let mode = &args[1];

    match mode.as_str() {
	"-c" => todo!("compile"), // compile
	_ => {
	    for s in args.iter().skip(1) {
		interpreter(&fs::read_to_string(s)
			    .expect("Should have been able to read the file"));
	    }
	} // read string
    }
}

fn interpreter(tokens: &str) {
    let mut tape: [u8; TAPE_LEN] = [0; TAPE_LEN];
    let mut stack: Vec<usize> = Vec::new();

    let mut tape_ptr: usize = 0;
    let mut stream = tokens.chars().enumerate();

    loop {
	let Some((ii, token)) = stream.next() else { break; };
	match token {
	    '>' if tape_ptr == TAPE_LEN - 1 => tape_ptr = 0,
	    '>' => tape_ptr += 1,

	    '<' if tape_ptr == 0 => tape_ptr = TAPE_LEN - 1,
	    '<' => tape_ptr -= 1,

	    '+' if tape[tape_ptr] == u8::MAX => tape[tape_ptr] = u8::MIN,
	    '+' => tape[tape_ptr] += 1,

	    '-' if tape[tape_ptr] == u8::MIN => tape[tape_ptr] = u8::MAX,
	    '-' => tape[tape_ptr] -= 1,

	    '.' => print!("{}", char::from(tape[tape_ptr])),
	    ',' => tape[tape_ptr] = {
		print!("input: ");
		let _ = stdout().flush();
		let input: Option<u8> = std::io::stdin()
		    .bytes() 
		    .next()
		    .and_then(|result| result.ok())
		    .map(|byte| byte as u8);
		print!("\n");
		input.unwrap()
	    },

	    '[' if tape[tape_ptr] == 0 => {
		let mut count: isize = 0;
		loop {
		    let Some((_, token)) = stream.next() else { panic!() };

		    match token {
			'[' => count += 1,
			']' if count == 0 => {
			    break;
			},
			']' if count < 0 => {
			    panic!("");
			},
			']' => count -= 1,
			_ => (),
		    }
		}
	    }, // jumps
	    '[' => {
		stack.push(ii - 1);
	    },

	    ']' if stack.is_empty() => panic!("No opening square bracket."),
	    ']' if tape[tape_ptr] != 0 => {
		stream = tokens.chars().enumerate();
		let index = stack.pop().expect("No opening square bracket.");
		let _ = stream.position(|(n, _)| n == index);
	    }, //jumps
	    ']' => {
		let _ = stack.pop();
	    },
	    _ => (),
	}
    }
}

