use crate::TAPE_LEN;

pub struct Tape {
    pub tape: [u8; TAPE_LEN],
    pub pos: usize,
}
