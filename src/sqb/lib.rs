mod asm;
mod disasm;

use hash40::{Hash40, ReadHash40, WriteHash40};
use std::path::Path;
use std::io::{Read, Write, Cursor, Error};

const MAGIC: &[u8; 4] = b"SQB\x00";

pub struct SequenceBank {
    pub unk: u16, // version number?
    pub sequences: Vec<Sequence>,
}

pub struct Sequence {
    pub seq_id: Hash40,
    pub unk1: u16,
    pub sounds: Vec<Sound>,
    pub unk2: u32,
}

pub struct Sound {
    pub sound_id: Hash40,
    pub unk1: u16,
    pub prob: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u32,
}

//pub fn open<P: AsRef<Path>>(filepath: P) -> Result<SequenceBank, Error> {

//}