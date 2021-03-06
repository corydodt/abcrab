use core::fmt;
use num::rational::Rational32;
use std::error::Error;
use std::fmt::{Display, Formatter};

mod pyabcrab;

#[derive(Debug)]
pub struct IllegalLength;

impl Error for IllegalLength {}

impl fmt::Display for IllegalLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This is not a legal time value for a note")
    }
}

pub mod parser;

#[derive(Debug, Clone, PartialEq)]
enum Accidental {
    Flat2,
    Flat,
    Natural,
    Sharp,
    Sharp2,
}

impl Display for Accidental {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Accidental::Flat2 => f.write_str("𝄫"),
            Accidental::Flat => f.write_str("♭"),
            Accidental::Natural => f.write_str("♮"),
            Accidental::Sharp => f.write_str("♯"),
            Accidental::Sharp2 => f.write_str("𝄪"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Note {
    octave: i32, // piano: 8 octaves, middle C is in the 4th octave
    key: char,
    accidental: Option<Accidental>,
    // FIXME: length should be isomorphic with ABC length
    length: Length,
}

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.key)?;
        if let Some(a) = &self.accidental {
            a.fmt(f)?
        };
        // write!(f, " ")?; self.length.fmt(f)?;
        write!(f, " {}", self.length)?;

        if self.octave != 4 {
            write!(f, " @{}>", self.octave)
        } else {
            write!(f, ">")
        }
    }
}

#[derive(Debug, PartialEq)]
enum NoteShape {
    Hundred28th,
    SixtyFourth,
    ThirtySecond,
    Sixteenth,
    Eighth,
    Quarter,
    Half,
    Whole,
    Breve,
}

impl Display for NoteShape {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Self::Hundred28th => "\u{1D164}",
            Self::SixtyFourth => "\u{1D163}",
            Self::ThirtySecond => "\u{1D162}",
            Self::Sixteenth => "\u{1D161}",
            Self::Eighth => "\u{1D160}",
            Self::Quarter => "\u{1D15F}",
            Self::Half => "\u{1D15E}",
            Self::Whole => "\u{1D15D}",
            Self::Breve => "\u{1D15C}",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq)]
pub struct Length {
    note_shape: NoteShape,
    dot: i32,
}

impl Length {
    pub fn new(ratio: (i32, i32)) -> Result<Length, IllegalLength> {
        // assign to a rational to simplify
        let rr = Rational32::new(ratio.0, ratio.1);
        let dur = match (rr.numer(), rr.denom()) {
            (1, 128) => Length {
                note_shape: NoteShape::Hundred28th,
                dot: 0,
            },
            (1, 64) => Length {
                note_shape: NoteShape::SixtyFourth,
                dot: 0,
            },
            (3, 128) => Length {
                note_shape: NoteShape::SixtyFourth,
                dot: 1,
            },
            (1, 32) => Length {
                note_shape: NoteShape::ThirtySecond,
                dot: 0,
            },
            // 5
            (3, 64) => Length {
                note_shape: NoteShape::ThirtySecond,
                dot: 1,
            },
            (7, 128) => Length {
                note_shape: NoteShape::ThirtySecond,
                dot: 2,
            },
            (1, 16) => Length {
                note_shape: NoteShape::Sixteenth,
                dot: 0,
            },
            // 9-11
            (3, 32) => Length {
                note_shape: NoteShape::Sixteenth,
                dot: 1,
            },
            // 13
            (7, 64) => Length {
                note_shape: NoteShape::Sixteenth,
                dot: 2,
            },
            (15, 128) => Length {
                note_shape: NoteShape::Sixteenth,
                dot: 3,
            },
            (1, 8) => Length {
                note_shape: NoteShape::Eighth,
                dot: 0,
            },
            // 17-23
            (3, 16) => Length {
                note_shape: NoteShape::Eighth,
                dot: 1,
            },
            // 25-27
            (7, 32) => Length {
                note_shape: NoteShape::Eighth,
                dot: 2,
            },
            // 29
            (15, 64) => Length {
                note_shape: NoteShape::Eighth,
                dot: 3,
            },
            // 31
            (1, 4) => Length {
                note_shape: NoteShape::Quarter,
                dot: 0,
            },
            // 33-47
            (3, 8) => Length {
                note_shape: NoteShape::Quarter,
                dot: 1,
            },
            // 49-55
            (7, 16) => Length {
                note_shape: NoteShape::Quarter,
                dot: 2,
            },
            // 57-59
            (15, 32) => Length {
                note_shape: NoteShape::Quarter,
                dot: 3,
            },
            // 61-63
            (1, 2) => Length {
                note_shape: NoteShape::Half,
                dot: 0,
            },
            // 65-95
            (3, 4) => Length {
                note_shape: NoteShape::Half,
                dot: 1,
            },
            // 97-111
            (7, 8) => Length {
                note_shape: NoteShape::Half,
                dot: 2,
            },
            // 113-119
            (15, 16) => Length {
                note_shape: NoteShape::Half,
                dot: 3,
            },
            // 121-127
            (1, 1) => Length {
                note_shape: NoteShape::Whole,
                dot: 0,
            },
            (3, 2) => Length {
                note_shape: NoteShape::Whole,
                dot: 1,
            },
            (7, 4) => Length {
                note_shape: NoteShape::Whole,
                dot: 2,
            },
            (15, 8) => Length {
                note_shape: NoteShape::Whole,
                dot: 3,
            },
            (2, 1) => Length {
                note_shape: NoteShape::Breve,
                dot: 0,
            },
            (3, 1) => Length {
                note_shape: NoteShape::Breve,
                dot: 1,
            },
            (7, 2) => Length {
                note_shape: NoteShape::Breve,
                dot: 2,
            },
            (15, 4) => Length {
                note_shape: NoteShape::Breve,
                dot: 3,
            },
            _ => return Err(IllegalLength {}),
        };
        Ok(dur)
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.note_shape.fmt(f)?;
        for _ in 0..(self.dot) {
            write!(f, "{}", DOT)?;
        }
        write!(f, "")
    }
}

const DOT: &str = "\u{1D16D}";
