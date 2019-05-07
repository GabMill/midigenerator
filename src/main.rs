//MIDI file generator. Creates MIDI files containing a desired chord or scale to drop into a
//DAW to play a software instrument.

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    {
        let e3: u8 = 0x40;
        let c3: u8 = 0x3C;
        let g2: u8 = 0x37;
        let c2: u8 = 0x30;
        let mut file = File::create("Cmaj.mid")?;
        // Write a slice of bytes to the file
        file.write_all(&[0x4D, 0x54, 0x68, 0x64, //MThd
            0x00, 0x00, 0x00, 0x06, //length
            0x00, 0x00, //format
            0x00, 0x01, //ntrks
            0x00, 0x60, //dividion
            0x4D, 0x54, 0x72, 0x6B, //MTrk (#1)
            0x00, 0x00, 0x00, 0x25, //length
            0x00, 0x90, e3, 0x63, //∆-time, note on channel(1), note 40, velocity 64
            0x00, 0x90, g2, 0x63, //∆-time, note on channel(1), note 37, velocity 64
            0x00, 0x90, c3, 0x63, //∆-time, note on channel(1), note 3C, velocity 64
            0x00, 0x90, c2, 0x63, //∆-time, note on channel(1), note 30, velocity 64
            0x83, 0x00, 0x80, e3, 0x00, //∆-time (two bytes), note off channel(1), note 40, velocity 0
            0x00, 0x80, c2, 0x00, //∆-time, note off channel(1), note 30, velocity 0
            0x00, 0x80, c3, 0x00, //∆-time, note off channel(1), note 3C, velocity 0
            0x00, 0x80, g2, 0x00, //∆-time, note off channel(1), note 37, velocity 0
            0x00, 0xFF, 0x2F, 0x00 //End of track
            ])?;
    }

    Ok(())
}
