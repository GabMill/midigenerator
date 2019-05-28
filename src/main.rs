//MIDI file generator. Creates MIDI files containing a desired chord or scale to drop into a
//DAW to play a software instrument.

use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::env;

enum Note {
    C = 0,
    Cs = 1,
    D = 2,
    Ds = 3,
    E = 4,
    F = 5,
    Fs = 6,
    G = 7,
    Gs = 8,
    A = 9,
    As = 10,
    B = 11,
}

fn map_scale(transposed_scale: &Vec<u8>, mapping: &str) -> Vec<u8> {
    let mut ret: Vec<u8> = transposed_scale.to_vec();
    match mapping {
        "major" => return ret,
        "minor" => {
            ret[2] -= 0x01;
            ret[5] -= 0x01;
            ret[6] -= 0x01;
            return ret;
        },
        "natural minor" => {
            ret[2] -= 0x01;
            ret[5] -= 0x01;
            ret[6] -= 0x01;
            return ret;
        },
        "harmonic minor" => {
            ret[2] -= 0x01;
            ret[5] -= 0x01;
            return ret;
        },
        "melodic minor" => {
            ret[2] -= 0x01;
            return ret;
        },
        _ => {
            println!("Bad scale mapping, providing major scale");
            return ret;
        },
    }
}

fn transpose(template_scale: &Vec<u8>, root: &str) -> Vec<u8> {
    let ret: Vec<u8> = template_scale.to_vec();
    match root {
        "C" => return ret,
        "Cs" => return ret.iter().map(|x| x + Note::Cs as u8).collect(),
        "D" => return ret.iter().map(|x| x + Note::D as u8).collect(),
        "Ds" => return ret.iter().map(|x| x + Note::Ds as u8).collect(),
        "E" => return ret.iter().map(|x| x + Note::E as u8).collect(),
        "F" => return ret.iter().map(|x| x + Note::F as u8).collect(),
        "Fs" => return ret.iter().map(|x| x + Note::Fs as u8).collect(),
        "G" => return ret.iter().map(|x| x + Note::G as u8).collect(),
        "Gs" => return ret.iter().map(|x| x + Note::Gs as u8).collect(),
        "A" => return ret.iter().map(|x| x + Note::A as u8).collect(),
        "As" => return ret.iter().map(|x| x + Note::As as u8).collect(),
        "B" => return ret.iter().map(|x| x + Note::B as u8).collect(),
        _ => {
            println!("Bad root, providing C scale");
            return ret;
        },
    }
}

fn create_chord_track(mut file: &File, notes: &[u8]) -> std::io::Result<()> {
    //Write track header
    file.write_all(&[
        0x4D, 0x54, 0x72, 0x6B, //MTrk
        0x00, 0x00, 0x00, 0x25, //length
        ])?;
    //Write start values for provided notes
    for x in 0 .. notes.len() {
        file.write_all(&[
            0x00, 0x90, notes[x], 0x63, //∆-time, note on channel(1), note 40, velocity 64
            ])?;
    }
    //Increase delta-time and add end for last note
    file.write_all(&[
            0x83, 0x00, 0x80, notes[0], 0x00, //∆-time (two bytes), note off channel(1), note 40, velocity 0
            ])?;
    //Write remaining notes
    for x in 1 .. notes.len() {
        file.write_all(&[
            0x00, 0x80, notes[x], 0x00, //∆-time, note off channel(1), note 30, velocity 0
            ])?;
    }
    //Write end of track
    file.write_all(&[
        0x00, 0xFF, 0x2F, 0x00 //End of track
        ])?;
    Ok(())
}

fn create_scale_track(mut file: &File, scale_temp: Vec<u8>) -> std::io::Result<()> {
    let length: u8 = scale_temp.len() as u8;
    file.write_all(&[
        0x4D, 0x54, 0x72, 0x6B, //MTrk
        0x00, 0x00, 0x00, (length * 0x09 + 0x0C), //length
        ])?;
    for x in 0 .. scale_temp.len() {
        file.write_all(&[
            0x00, 0x90, scale_temp[x], 0x63, //∆-time, note on channel(1), note, velocity 64
            0x83, 0x00, 0x80, scale_temp[x], 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
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

fn print_usage_message() {
    println!("Too few arguments");
    println!("Usage: midigenerator (scale | chord) <key> <mapping>");
    println!("Available scale mappings:\nmajor\n(natural) minor\nharmonic minor\nmelodic minor\n");
}

fn main() -> std::io::Result<()> {
    {
        //Create file
        let file = File::create("Cmaj.mid")?;
        //Add header as described by MIDI standard (one track, one channel)
        add_header(&file)?;
        //Create "template scale", which is a C major scale starting at C0
        let default = vec![0x30, 0x32, 0x34, 0x35, 0x37, 0x39, 0x3B, 0x3C];
        let args: Vec<String> = env::args().collect();
        if args.len() < 4 {
            print_usage_message();
            std::process::exit(0);
        }
        let op = &args[1];
        let root = &args[2];
        let map_to = &args[3];
        match op.as_str() {
            "chord" => println!("Chord generation goes here"),
            "scale" => {
                let transposed: Vec<u8> = transpose(&default, root);
                println!("{:?}", transposed);
                let mapped: Vec<u8> = map_scale(&transposed, map_to);
                create_scale_track(&file, mapped)?;
            },
            _ => println!("Default"),
        }
    }
    Ok(())
}
