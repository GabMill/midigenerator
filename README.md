# midigenerator

A tool for creating midi files containing scales and chords to use for playing
software instruments.

# Usage
Midigenerator has two modes - it can create a chord or a scale. To create a
C major chord for example, the usage would be

midigenerator c C major

The 'c' tells midigenerator that it is making a chord, 'C' gives it the key,
and 'major' tells it how to map the chord. Similarly, a scale or mode can be
produced as follows:

midignerator s Ds natural_minor

's' tells it that we want a scale, 'Ds' indicates we want the key of D sharp
(only sharps are implemented, no flats), and 'natural_minor' provides the scale
mapping that is desired. The following chords and scales that are available are
listed below.

Chords:
maj, m/min, 7, m7, maj7, minM7/mM7, 6, m6, 6/9, 5, 9, m9, maj9, 11, m11, maj11,
13, m13, add, 7-5, 7+5, sus, dim, dim7, m7b5, aug, aug7

Scales:
major, minor/natural_minor, harmonic_minor, melodic_minor, major pentatonic/pentatonic,
minor pentatonic, all modes (dorian, phrygian, lydian, etc.)
