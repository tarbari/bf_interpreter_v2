use crate::instructions::*;
use crate::program::Program;
use crate::tape::Tape;
use std::env;

mod instructions;
mod program;
mod tape;

const TAPE_LEN: usize = 3000; // Create 3000 bytes to manipulate

fn main() {
    let file_name = parse_args();
    let program = read_file(file_name);

    let mut t = Tape {
        tape: [0; TAPE_LEN],
        pos: 0,
    };

    let mut p = Program {
        program,
        pos: 0,
        bracket_counter: 0,
    };

    while p.pos < p.program.len() {
        let i = p.program[p.pos];
        match i {
            '+' => {
                increment_byte(&mut t);
            }
            '-' => {
                decrement_byte(&mut t);
            }
            '>' => {
                t_pos_next(&mut t);
            }
            '<' => {
                t_pos_previous(&mut t);
            }
            '.' => {
                print(&mut t);
            }
            ',' => {
                input(&mut t);
            }
            '[' => {
                start_loop(&mut t, &mut p);
            }
            ']' => {
                end_loop(&mut t, &mut p);
            }
            _ => {}
        }
        p.pos += 1;
    }
}

fn read_file(file_name: String) -> Vec<char> {
    // TODO: Needs error handling
    let res: Vec<char> = std::fs::read_to_string(file_name)
        .unwrap()
        .chars()
        .collect();

    res
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    args[1].to_string()
}
