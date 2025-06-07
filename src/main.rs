// SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::{
    Read,
    Write,
    stdout,
    stdin,
    IsTerminal,
};

use std::env;
use std::fs;

const TAPE_LEN: usize = 30_000;

macro_rules! interpreter {
    ($t:expr) => {
	interpreter($t,
		    &mut [0; TAPE_LEN],
		    &mut Vec::<usize>::new(),
		    &mut 0
	);
    };
    ($t:expr, $m:expr, $s:expr, $p:expr) => {
	interpreter($t, $m, $s, $p)
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
	let mut m: [u8; TAPE_LEN] = [0; TAPE_LEN];
	let mut s: Vec<usize> = Vec::new();
	let mut p: usize = 0;
	loop {
	    print!("[{} | 0x{:X}] ==> ", p, m[p]);
	    let _ = stdout().flush();
	    let mut input = String::new();
	    match stdin().read_line(&mut input) {
		Ok(..) => {
		    interpreter!(&input, &mut m, &mut s, &mut p);
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
		interpreter!(&fs::read_to_string(s)
			     .expect("Should have been able to read the file"));
	    }
	} // read string
    }
}

fn interpreter(tokens: &str, tape: &mut [u8; TAPE_LEN], stack: &mut Vec<usize>, tape_ptr: &mut usize) {
    let mut stream = tokens.chars().enumerate();

    loop {
	let Some((ii, token)) = stream.next() else { break; };

	match token {
	    '>' if *tape_ptr == TAPE_LEN - 1 => *tape_ptr = 0,
	    '>' => *tape_ptr += 1,

	    '<' if *tape_ptr == 0 => *tape_ptr = TAPE_LEN - 1,
	    '<' => *tape_ptr -= 1,

	    '+' if tape[*tape_ptr] == u8::MAX => tape[*tape_ptr] = u8::MIN,
	    '+' => tape[*tape_ptr] += 1,

	    '-' if tape[*tape_ptr] == u8::MIN => tape[*tape_ptr] = u8::MAX,
	    '-' => tape[*tape_ptr] -= 1,

	    '.' => print!("{}", char::from(tape[*tape_ptr])),
	    ',' => tape[*tape_ptr] = {
		let input = stdin();
		if !input.is_terminal() {
		    print!("input: ");
		    let _ = stdout().flush();
		}
		let byte: Option<u8> = input.bytes().next().and_then(|result| result.ok());

		byte.unwrap()
	    },

	    '[' if tape[*tape_ptr] == 0 => {
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
	    ']' if tape[*tape_ptr] != 0 => {
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

