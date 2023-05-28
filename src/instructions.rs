use crate::program::Program;
use crate::tape::Tape;
use crate::TAPE_LEN;
use std::io::Read;

pub fn increment_byte(t: &mut Tape) {
    if t.tape[t.pos] < 255 {
        t.tape[t.pos] += 1;
    } else {
        t.tape[t.pos] = 0;
    }
}

pub fn decrement_byte(t: &mut Tape) {
    if t.tape[t.pos] > 0 {
        t.tape[t.pos] -= 1;
    } else {
        t.tape[t.pos] = 255;
    }
}

pub fn t_pos_next(t: &mut Tape) {
    if t.pos < TAPE_LEN - 1 {
        t.pos += 1;
    } else {
        t.pos = 0;
    }
}

pub fn t_pos_previous(t: &mut Tape) {
    if t.pos > 0 {
        t.pos -= 1;
    } else {
        t.pos = TAPE_LEN - 1;
    }
}

pub fn print(t: &mut Tape) {
    print!("{}", t.tape[t.pos] as char);
}

pub fn input(t: &mut Tape) {
    // TODO: This needs some error handling. Also need a way of not needing to hit enter to input.
    // e.g. when a non ascii character is input the output is undefined
    t.tape[t.pos] = std::io::stdin()
        .bytes()
        .next()
        .and_then(|res| res.ok())
        .unwrap();
}

pub fn start_loop(t: &mut Tape, p: &mut Program) {
    if t.tape[t.pos] != 0 {
        return;
    }
    p.pos += 1;
    loop {
        if p.program[p.pos] == ']' && p.bracket_counter == 0 {
            break;
        } else if p.program[p.pos] == '[' {
            p.bracket_counter += 1;
        } else if p.program[p.pos] == ']' {
            p.bracket_counter -= 1;
        }
        p.pos += 1;
    }
}

pub fn end_loop(t: &mut Tape, p: &mut Program) {
    if t.tape[t.pos] == 0 {
        return;
    }
    p.pos -= 1;
    loop {
        if p.program[p.pos] == '[' && p.bracket_counter == 0 {
            break;
        } else if p.program[p.pos] == ']' {
            p.bracket_counter += 1;
        } else if p.program[p.pos] == '[' {
            p.bracket_counter -= 1;
        }
        p.pos -= 1;
    }
}
