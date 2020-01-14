use crate::{SequenceBank, Sequence, Sound, MAGIC};
use byteorder::{LittleEndian, WriteBytesExt};
use hash40::WriteHash40;
use std::io::{Write, Seek, Error, SeekFrom};

pub fn assemble<C: Write + Seek>(writer: &mut C, sqb: &SequenceBank) -> Result<(), Error> {
    writer.write_all(MAGIC)?;
    writer.write_u16::<LittleEndian>(sqb.unk)?;
    let len = sqb.sequences.len() as u16;
    let table_size = 4 * len as u32;
    writer.write_u16::<LittleEndian>(len)?;
    writer.write_u32::<LittleEndian>(table_size)?;

    let mut table_pos = writer.seek(SeekFrom::Current(0))?;
    let mut seq_pos = table_pos + table_size as u64;
    let seq_start = seq_pos;

    for seq in &sqb.sequences {
        writer.seek(SeekFrom::Start(table_pos))?;
        writer.write_u32::<LittleEndian>((seq_pos - seq_start) as u32)?;
        table_pos += 4;
        writer.seek(SeekFrom::Start(seq_pos))?;
        write_sequence(writer, seq)?;
        seq_pos = writer.seek(SeekFrom::Current(0))?;
    }

    Ok(())
}

pub fn write_sequence<C: Write + Seek>(writer: &mut C, seq: &Sequence) -> Result<(), Error> {
    writer.write_hash40::<LittleEndian>(seq.id)?;
    writer.write_u16::<LittleEndian>(seq.unk)?;
    writer.write_u16::<LittleEndian>(seq.sounds.len() as u16)?;
    writer.write_u32::<LittleEndian>(0)?; // padding
    for sound in &seq.sounds {
        write_sound(writer, &sound)?;
    }
    Ok(())
}

pub fn write_sound<C: Write + Seek>(writer: &mut C, sound: &Sound) -> Result<(), Error> {
    writer.write_hash40::<LittleEndian>(sound.id)?;
    writer.write_u16::<LittleEndian>(sound.unk1)?;
    writer.write_u16::<LittleEndian>(sound.prob)?;
    writer.write_i16::<LittleEndian>(sound.unk2)?;
    writer.write_i16::<LittleEndian>(sound.unk3)?;
    writer.write_u32::<LittleEndian>(sound.unk4)?;
    Ok(())
}
