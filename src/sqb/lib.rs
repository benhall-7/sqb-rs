mod asm;
mod disasm;

use hash40::Hash40;
use serde::{Deserialize, Serialize};
use std::fs::{read, write};
use std::io::{Cursor, Error, Read, Seek, Write};
use std::path::Path;

pub use hash40;

const MAGIC: &[u8; 4] = b"SQB\x00";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SequenceBank {
    pub unk: u16, // version number?
    pub sequences: Vec<Sequence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sequence {
    pub id: Hash40,
    pub unk: u16,
    pub sounds: Vec<Sound>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sound {
    pub id: Hash40,
    pub unk1: u16,
    pub prob: u16,
    pub unk2: i16,
    pub unk3: i16,
    pub unk4: u32,
}

pub fn from_stream<C: Read + Seek>(reader: &mut C) -> Result<SequenceBank, Error> {
    disasm::disassemble(reader)
}

pub fn to_stream<C: Write + Seek>(writer: &mut C, sqb: &SequenceBank) -> Result<(), Error> {
    asm::assemble(writer, sqb)
}

pub fn open<P: AsRef<Path>>(filepath: P) -> Result<SequenceBank, Error> {
    let mut cursor = Cursor::new(read(filepath)?);
    disasm::disassemble(&mut cursor)
}

pub fn save<P: AsRef<Path>>(filepath: P, sqb: &SequenceBank) -> Result<(), Error> {
    let mut writer = Cursor::new(Vec::<u8>::new());
    asm::assemble(&mut writer, sqb)?;
    write(filepath, &writer.into_inner())
}
