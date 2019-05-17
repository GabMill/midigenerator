//MIDI file generator. Creates MIDI files containing a desired chord or scale to drop into a
//DAW to play a software instrument.

use std::fs::File;
use std::io::prelude::*;
use std::env;

const C: u8 = 0x00;
const CS: u8 = 0x01;
const D: u8 = 0x02;
const DS: u8 = 0x03;
const E: u8 = 0x04;
const F: u8 = 0x05;
const FS: u8 = 0x06;
const G: u8 = 0x07;
const GS: u8 = 0x08;
const A: u8 = 0x09;
const AS: u8 = 0x0A;
const B: u8 = 0x0B;

fn create_triad(mut file: File, root: u8) -> File {
    let fourth = root + 0x04 + 0x0C;
    let fifth = root + 0x07;
    file.write_all(&[
        0x4D, 0x54, 0x72, 0x6B, //MTrk
        0x00, 0x00, 0x00, 0x25, //length
        0x00, 0x90, fourth, 0x63, //∆-time, note on channel(1), note 40, velocity 64
        0x00, 0x90, fifth, 0x63, //∆-time, note on channel(1), note 37, velocity 64
        0x00, 0x90, (root + 0x0c), 0x63, //∆-time, note on channel(1), note 3C, velocity 64
        0x00, 0x90, root, 0x63, //∆-time, note on channel(1), note 30, velocity 64
        0x83, 0x00, 0x80, fourth, 0x00, //∆-time (two bytes), note off channel(1), note 40, velocity 0
        0x00, 0x80, root, 0x00, //∆-time, note off channel(1), note 30, velocity 0
        0x00, 0x80, (root + 0x0C), 0x00, //∆-time, note off channel(1), note 3C, velocity 0
        0x00, 0x80, fifth, 0x00, //∆-time, note off channel(1), note 37, velocity 0
        0x00, 0xFF, 0x2F, 0x00 //End of track
        ]);
    return file
}

fn create_scale(mut file: File, root: u8) -> File {
    let mut note = root;
    file.write_all(&[
        0x4D, 0x54, 0x72, 0x6B, //MTrk
        0x00, 0x00, 0x00, 0x4C, //length
        0x00, 0x90, note, 0x63, //∆-time, note on channel(1), note, velocity 64
        0x83, 0x00, 0x80, note, 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        0x00, 0x90, (note + 0x02), 0x63, //∆-time, note on channel(1), note, velocity 64
        0x83, 0x00, 0x80, (note + 0x02), 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        0x00, 0x90, (note + 0x04), 0x63, //∆-time, note on channel(1), note, velocity 64
        0x83, 0x00, 0x80, (note + 0x04), 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        0x00, 0x90, (note + 0x05), 0x63, //∆-time, note on channel(1), note, velocity 64
        0x83, 0x00, 0x80, (note + 0x05), 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        0x00, 0x90, (note + 0x07), 0x63, //∆-time, note on channel(1), note, velocity 64
        0x83, 0x00, 0x80, (note + 0x07), 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        0x00, 0x90, (note + 0x09), 0x63, //∆-time, note on channel(1), note, velocity 64
        0x83, 0x00, 0x80, (note + 0x09), 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        0x00, 0x90, (note + 0x0B), 0x63, //∆-time, note on channel(1), note, velocity 64
        0x83, 0x00, 0x80, (note + 0x0B), 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        0x00, 0x90, (note + 0x0C), 0x63, //∆-time, note on channel(1), note, velocity 64
        0x83, 0x00, 0x80, (note + 0x0C), 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        0x00, 0xFF, 0x2F, 0x00 //End of track
        ]);
    return file
}

fn add_header(mut file: File) -> File {
    file.write_all(&[
        0x4D, 0x54, 0x68, 0x64, //MThd
        0x00, 0x00, 0x00, 0x06, //length
        0x00, 0x00, //format
        0x00, 0x01, //ntrks
        0x00, 0x60, //division
        ]);
    return file
}

fn main() -> std::io::Result<()> {
    {
        let major = vec![0x02, 0x02, 0x01, 0x02, 0x02, 0x02, 0x01];
        let args: Vec<String> = env::args().collect();
        let mut file = File::create("Cmaj.mid")?;
        file = add_header(file);
        // file = create_triad(file, C);
        // file = create_triad(file, E);
        // file = create_triad(file, GS);
        file = create_scale(file, C);
        file = create_scale(file, F);
    }
    Ok(())
}
