//MIDI file generator. Creates MIDI files containing a desired chord or scale to drop into a
//DAW to play a software instrument.

use std::env;
use std::fs::File;
use std::io::prelude::*;

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
        "major" | "ionian" => return ret,
        "minor" | "natural_minor" | "aeolian" => {
            ret[2] -= 1;
            ret[5] -= 1;
            ret[6] -= 1;
        }
        "harmonic_minor" => {
            ret[2] -= 1;
            ret[5] -= 1;
        }
        "melodic_minor" => {
            ret[2] -= 1;
        }
        "dorian" => {
            ret[2] -= 1;
            ret[6] -= 1;
        }
        "phrygian" => {
            ret[1] -= 1;
            ret[2] -= 1;
            ret[5] -= 1;
            ret[6] -= 1;
        }
        "lydian" => {
            ret[3] += 1;
        }
        "mixolydian" => {
            ret[6] -= 1;
        }
        "locrian" => {
            ret[1] -= 1;
            ret[2] -= 1;
            ret[4] -= 1;
            ret[5] -= 1;
            ret[6] -= 1;
        }
        "pentatonic" | "major_pentatonic" => {
            ret.remove(6);
            ret.remove(2);
        }
        "minor_pentatonic" => {
            ret.remove(6);
            ret.remove(2);
            ret[1] += 1;
            ret[4] += 1;
        }
        _ => {
            println!(
                "Scale mapping \"{}\" not recognized, providing default major scale",
                mapping
            );
        }
    }
    ret
}

fn map_chord(transposed_scale: &Vec<u8>, mapping: &str) -> Vec<u8> {
    let mut ret: Vec<u8> = transposed_scale.to_vec();
    ret.remove(7); //Remove octave by defualt
    match mapping {
        //Match provided chord mapping and remove notes and change intervals
        //to create chord.
        "maj" => {
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "m" | "min" => {
            ret[2] -= 1; //adjust degree to minor 3rd
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "7" => {
            ret[6] -= 1; //adjust degree to minor 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "m7" => {
            ret[6] -= 1; //adjust degree to minor 7th
            ret[2] -= 1; //adjust degree to minor 3rd
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "maj7" => {
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "minM7" | "mM7" => {
            ret[2] -= 1; //adjust degree to minor 3rd
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "6" => {
            ret.remove(6); //remove 7th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "m6" => {
            ret[2] -= 1; //adjust degree to minor 3rd
            ret.remove(6); //remove 7th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "6/9" => {
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret.remove(6); //remove 7th
            ret.remove(3); //remove 4th
        }
        "5" => {
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(2); //remove 3rd
            ret.remove(1); //remove 2nd
        }
        "9" => {
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret[6] -= 1; //adjust degree to minor 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
        }
        "m9" => {
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret[2] -= 1; //adjust degree to minor 3rd
            ret[6] -= 1; //adjust degree to minor 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
        }
        "maj9" => {
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
        }
        "11" => {
            ret[3] += 12; //adjust 4th up an octave to 11th
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret[6] -= 1; //adjust degree to minor 7th
            ret.remove(5); //remove 6th
        }
        "m11" => {
            ret[3] += 12; //adjust 4th up an octave to 11th
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret[2] -= 1; //adjust degree to minor 3rd
            ret[6] -= 1; //adjust degree to minor 7th
            ret.remove(5); //remove 6th
        }
        "maj11" => {
            ret[3] += 12; //adjust 4th up an octave to 11th
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret.remove(5); //remove 6th
        }
        "13" => {
            ret[5] += 12; //adjust 6th up an octave to 13th
            ret[3] += 12; //adjust 4th up an octave to 11th
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret[6] -= 1; //adjust degree to minor 7th
        }
        "m13" => {
            ret[3] += 12; //adjust 4th up an octave to 11th
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret[2] -= 1; //adjust degree to minor 3rd
            ret[6] -= 1; //adjust degree to minor 7th
            ret.remove(5); //remove 6th
        }
        "maj13" => {
            ret[5] += 12; //adjust 6th up an octave to 13th
            ret[3] += 12; //adjust 4th up an octave to 11th
            ret[1] += 12; //adjust 2nd up an octave to 9th
        }
        "add2" => {
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "add9" => {
            ret[1] += 12; //adjust 2nd up an octave to 9th
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
        }
        "7b5" => {
            ret[6] -= 1; //adjust degree to minor 7th
            ret[4] -= 1; //flat fifth
            ret[6] -= 1; //adjust degree to minor 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "7s5" => {
            ret[6] -= 1; //adjust degree to minor 7th
            ret[4] -= 1; //sharp fifth
            ret[6] -= 1; //adjust degree to minor 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "sus2" => {
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(2); //remove 3rd
        }
        "sus4" => {
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(2); //remove 3rd
            ret.remove(1); //remove 2nd
        }
        "dim" => {
            ret[2] -= 1;
            ret[4] -= 1;
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "dim7" => {
            ret[2] -= 1;
            ret[4] -= 1;
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "m7b5" => {
            ret[6] -= 1; //adjust degree to minor 7th
            ret[4] -= 1; //adjust degree to minor 5th
            ret[2] -= 1; //adjust degree to minor 3rd
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "aug" => {
            ret[4] += 1; //augment fifth
            ret.remove(6); //remove 7th
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        "aug7" => {
            ret[6] -= 1; //adjust degree to minor 7thd
            ret[4] += 1; //augment fifth
            ret.remove(5); //remove 6th
            ret.remove(3); //remove 4th
            ret.remove(1); //remove 2nd
        }
        //Default
        _ => {
            println!("Provided chord mapping not recognized, returning major triad");
            ret.remove(6);
            ret.remove(5);
            ret.remove(3);
            ret.remove(1);
        }
    }
    ret
}

fn transpose_scale(template_scale: &Vec<u8>, root: &str) -> Vec<u8> {
    let ret: Vec<u8> = template_scale.to_vec();
    match root {
        "C" => ret,
        "Cs" => ret.iter().map(|x| x + Note::Cs as u8).collect(),
        "D" => ret.iter().map(|x| x + Note::D as u8).collect(),
        "Ds" => ret.iter().map(|x| x + Note::Ds as u8).collect(),
        "E" => ret.iter().map(|x| x + Note::E as u8).collect(),
        "F" => ret.iter().map(|x| x + Note::F as u8).collect(),
        "Fs" => ret.iter().map(|x| x + Note::Fs as u8).collect(),
        "G" => ret.iter().map(|x| x + Note::G as u8).collect(),
        "Gs" => ret.iter().map(|x| x + Note::Gs as u8).collect(),
        "A" => ret.iter().map(|x| x + Note::A as u8).collect(),
        "As" => ret.iter().map(|x| x + Note::As as u8).collect(),
        "B" => ret.iter().map(|x| x + Note::B as u8).collect(),
        _ => {
            println!("\"{}\" root not recognized, providing C scale", root);
            ret
        }
    }
}

fn create_chord_track(mut file: &File, notes: Vec<u8>) -> std::io::Result<()> {
    let length: u8 = notes.len() as u8;
    //Write track header
    file.write_all(&[
        0x4D,
        0x54,
        0x72,
        0x6B, //MTrk
        0x00,
        0x00,
        0x00,
        (length * 8 + 0x10), //length
    ])?;
    //Write start values for provided notes
    for x in &notes {
        file.write_all(&[
            0x00, 0x90, *x, 0x63, //∆-time, note on channel(1), note 40, velocity 64
        ])?;
    }
    //Increase delta-time and add end for last note
    file.write_all(&[
        0x83, 0x00, 0x80, notes[0],
        0x00, //∆-time (two bytes), note off channel(1), note 40, velocity 0
    ])?;
    //Write remaining notes
    for x in 1..notes.len() {
        file.write_all(&[
            0x00, 0x80, notes[x], 0x00, //∆-time, note off channel(1), note 30, velocity 0
        ])?;
    }
    //Write end of track
    file.write_all(&[
        0x00, 0xFF, 0x2F, 0x00, //End of track
    ])?;
    Ok(())
}

fn create_scale_track(mut file: &File, scale_temp: Vec<u8>) -> std::io::Result<()> {
    let length: u8 = scale_temp.len() as u8;
    file.write_all(&[
        0x4D, 0x54, 0x72, 0x6B, //MTrk
        0x00, 0x00, 0x00, (length * 0x09 + 0x0C), //length
    ])?;
    for x in 0..scale_temp.len() {
        file.write_all(&[
            0x00, 0x90, scale_temp[x], 0x63, //∆-time, note on channel(1), note, velocity 64
            0x83, 0x00, 0x80, scale_temp[x], 0x00, //∆-time (two bytes), note off channel(1), note, velocity 0
        ])?;
    }
    file.write_all(&[
        0x00, 0xFF, 0x2F, 0x00, //End of track
    ])?;
    Ok(())
}

fn add_midi_header(mut file: &File) -> std::io::Result<()> {
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
    print!("Too few arguments\n");
    print!("Usage: midigenerator (s(cale) | c(hord)) <key> <mapping>\n\n");
    print!("If no mapping is provided or is not recognized, default is major\n");
    print!("Denote sharps by adding an s to the key (C sharp would be Cs)\n\n");
    print!("Available scale mappings: major (natural_)minor ");
    print!("harmonic_minor melodic_minor\n(major_)pentatonic minor_pentatonic ");
    print!("All modes\n\nChord mappings: maj, m/min, 7, m7, maj7, mM7/minM7, 6, m6,\n");
    print!("6/9, 5, 9, m9, maj9, 11, m11, maj11, 13, m13, maj13, add2, add9, 7-5, 7+5, sus2,\n");
    print!("sus4, dim, dim7, m7b5, aug, aug7\n");
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    //Create "template scale", which is a C major scale starting at C0
    let default = vec![
        (Note::C as u8 + 36),
        (Note::D as u8 + 36),
        (Note::E as u8 + 36),
        (Note::F as u8 + 36),
        (Note::G as u8 + 36),
        (Note::A as u8 + 36),
        (Note::B as u8 + 36),
        (Note::C as u8 + 48),
    ];
    match args.len() {
        1 | 2 => {
            print_usage_message();
            return Ok(());
        }
        3 => {
            let op = &args[1];
            let root = &args[2];
            let mut fname = String::new();
            fname.push_str(root);
            fname.push_str("maj.mid");
            //Create file
            let file = File::create(&fname)?;
            //Add header as described by MIDI standard (one track, one channel)
            add_midi_header(&file)?;
            match op.as_str() {
                "c" => {
                    let transposed: Vec<u8> = transpose_scale(&default, root);
                    let mapped: Vec<u8> = map_chord(&transposed, "maj");
                    create_chord_track(&file, mapped)?
                }
                "s" => {
                    let transposed: Vec<u8> = transpose_scale(&default, root);
                    let mapped: Vec<u8> = map_scale(&transposed, "maj");
                    create_scale_track(&file, mapped)?;
                }
                _ => {
                    print_usage_message();
                    return Ok(());
                }
            }
            println!("Wrote {:?}", &fname)
        }
        4 => {
            let op = &args[1];
            let root = &args[2];
            let map_to = &args[3];
            let mut fname = String::new();
            fname.push_str(root);
            fname.push_str(map_to);
            fname.push_str(".mid");
            //Create file
            let file = File::create(&fname)?;
            //Add header as described by MIDI standard (one track, one channel)
            add_midi_header(&file)?;
            match op.as_str() {
                "c" => {
                    let transposed: Vec<u8> = transpose_scale(&default, root);
                    let mapped: Vec<u8> = map_chord(&transposed, map_to);
                    create_chord_track(&file, mapped)?
                }
                "s" => {
                    let transposed: Vec<u8> = transpose_scale(&default, root);
                    let mapped: Vec<u8> = map_scale(&transposed, map_to);
                    create_scale_track(&file, mapped)?;
                }
                _ => {
                    print_usage_message();
                    return Ok(());
                }
            }
            println!("Wrote {:?}", &fname)
        }
        _ => {
            print_usage_message();
            return Ok(());
        }
    }
    Ok(())
}
