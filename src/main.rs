// SPDX-FileCopyrightText: 2025 Imran M <imran@imranmustafa.net>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::{
    Read,
    Write,
    stdout
};

const TAPE_LEN: usize = 30_000;

fn main() {
    interpreter("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
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
		loop {
		    let Some((_, token)) = stream.next() else {panic!()};
		    let mut count: usize = 0;

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

