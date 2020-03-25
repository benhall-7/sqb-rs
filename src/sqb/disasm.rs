use crate::{Sequence, SequenceBank, Sound, MAGIC};
use byteorder::{LittleEndian, ReadBytesExt};
use hash40::ReadHash40;
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom};

pub fn disassemble<C: Read + Seek>(reader: &mut C) -> Result<SequenceBank, Error> {
    let mut magic_bytes = [0; 4];
    reader.read_exact(&mut magic_bytes)?;
    if &magic_bytes != MAGIC {
        return Err(Error::new(ErrorKind::InvalidData, "Invalid file magic"));
    }

    let unk = reader.read_u16::<LittleEndian>()?;
    let count = reader.read_u16::<LittleEndian>()?;
    let size = reader.read_u32::<LittleEndian>()?;
    let table_offset = reader.seek(SeekFrom::Current(0))?;
    let start_offset = table_offset + size as u64;

    let sequences = (0..count)
        .map(|i| {
            reader.seek(SeekFrom::Start(table_offset + 4 * i as u64))?;
            let offset = reader.read_u32::<LittleEndian>()?;
            reader.seek(SeekFrom::Start(start_offset + offset as u64))?;
            read_seq(reader)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(SequenceBank { unk, sequences })
}

fn read_seq<C: Read + Seek>(reader: &mut C) -> Result<Sequence, Error> {
    let id = reader.read_hash40::<LittleEndian>()?;
    let unk = reader.read_u16::<LittleEndian>()?;
    let count = reader.read_u16::<LittleEndian>()?;
    reader.read_u32::<LittleEndian>()?; //padding;

    let sounds = (0..count)
        .map(|_| read_sound(reader))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Sequence { id, unk, sounds })
}

fn read_sound<C: Read + Seek>(reader: &mut C) -> Result<Sound, Error> {
    let id = reader.read_hash40::<LittleEndian>()?;
    let unk1 = reader.read_u16::<LittleEndian>()?;
    let prob = reader.read_u16::<LittleEndian>()?;
    let unk2 = reader.read_i16::<LittleEndian>()?;
    let unk3 = reader.read_i16::<LittleEndian>()?;
    let unk4 = reader.read_u32::<LittleEndian>()?;
    Ok(Sound {
        id,
        unk1,
        prob,
        unk2,
        unk3,
        unk4,
    })
}
