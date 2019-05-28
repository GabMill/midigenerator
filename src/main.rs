//MIDI file generator. Creates MIDI files containing a desired chord or scale to drop into a
//DAW to play a software instrument.

use std::fs::File;
use std::io::prelude::*;
use std::env;

fn create_triad(mut file: &File, root: u8) -> std::io::Result<()> {
    let fourth = root + 0x04 + 0x0C;
    let fifth = root + 0x07;
    file.write_all(&[
        0x4D, 0x54, 0x72, 0x6B, //MTrk
        0x00, 0x00, 0x00, 0x25, //length
        0x00, 0x90, fourth, 0x63, //∆-time, note on channel(1), note 40, velocity 64
        0x00, 0x90, fifth, 0x63, //∆-time, note on channel(1), note 37, velocity 64
        0x00, 0x90, (root + 0x0C), 0x63, //∆-time, note on channel(1), note 3C, velocity 64
        0x00, 0x90, root, 0x63, //∆-time, note on channel(1), note 30, velocity 64
        0x83, 0x00, 0x80, fourth, 0x00, //∆-time (two bytes), note off channel(1), note 40, velocity 0
        0x00, 0x80, root, 0x00, //∆-time, note off channel(1), note 30, velocity 0
        0x00, 0x80, (root + 0x0C), 0x00, //∆-time, note off channel(1), note 3C, velocity 0
        0x00, 0x80, fifth, 0x00, //∆-time, note off channel(1), note 37, velocity 0
        0x00, 0xFF, 0x2F, 0x00 //End of track
        ])?;
    Ok(())
}

fn create_scale(mut file: &File, scale: &Vec<u8>) -> std::io::Result<()> {
    let length: u8 = scale.len() as u8;
    file.write_all(&[
        0x4D, 0x54, 0x72, 0x6B, //MTrk
        0x00, 0x00, 0x00, length * 0x09 + 0x0C, //length
        ])?;
    for x in 0 .. scale.len() {
        file.write_all(&[
            0x00, 0x90, scale[x], 0x63, //∆-time, note on channel(1), note, velocity 64
            0x83, 0x00, 0x80, scale[x], 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
            ])?;
    }
    file.write_all(&[
        0x00, 0xFF, 0x2F, 0x00 //End of track
        ])?;
    Ok(())
}

fn add_header(mut file: &File) -> std::io::Result<()> {
    file.write_all(&[
        0x4D, 0x54, 0x68, 0x64, //MThd
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x00, //format
        0x00, 0x01, //ntrks
        0x00, 0x60, //division
        ])?;
    Ok(())
}


fn main() -> std::io::Result<()> {
    {
        let default = vec![0x00, 0x02, 0x04, 0x05, 0x07, 0x09, 0x0B, 0x0C];
        let transposed: Vec<u8> = default.iter().map(|x| x + 0x30 as u8).collect();
        let args: Vec<String> = env::args().collect();
        let file = File::create("Cmaj.mid")?;
        add_header(&file)?;
        create_scale(&file, &transposed)?;
    }
    Ok(())
}
