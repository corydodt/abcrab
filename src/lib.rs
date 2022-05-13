use core::fmt;
use std::fmt::{Display, Formatter};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::satisfy,
    combinator::opt,
    combinator::{map, value},
    multi::fold_many0,
    multi::many1_count,
    sequence::tuple,
    IResult,
};

mod pyabcrab;

#[derive(Debug, Clone, PartialEq)]
pub enum Accidental {
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
    length: i32, // power of 2, where 0 represents a quarter note (or note of length `L:`); constrained to be within -5 to 10
}

impl Note {
    pub fn new(octave: i32, key: char, accidental: Option<Accidental>, length: i32) -> Note {
        Note {
            octave,
            key,
            accidental,
            length,
        }
    }
}

// Example: middle C# whole note with L:1/4 is encoded as:
// Note { 4, KeyName::C, Some(Accidental::Sharp), 2 }
//
// Written in ABC notation this is ^C4 (assuming L:1/4)

const _DOTTED: &str = "\u{1D16D}";

const NOTES: [&str; 9] = [
    "\u{1D164}", // 1/128th note
    "\u{1D163}",
    "\u{1D162}",
    "\u{1D161}",
    "\u{1D160}",
    "\u{1D15F}", // quarter note
    "\u{1D15E}",
    "\u{1D15D}", // whole note
    "\u{1D15C}", // breve
];

impl Display for Note {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}", self.key)?;
        if let Some(a) = &self.accidental {
            a.fmt(f)?
        };
        write!(f, " {}", NOTES[(3 + self.length) as usize])?;

        if self.octave != 4 {
            write!(f, " @{}>", self.octave)
        } else {
            write!(f, ">")
        }
    }
}

pub fn accidental(input: &str) -> IResult<&str, Accidental> {
    let flatflat = value(Accidental::Flat2, tag("__"));
    let flat = value(Accidental::Flat, tag("_"));
    let sharpsharp = value(Accidental::Sharp2, tag("^^"));
    let sharp = value(Accidental::Sharp, tag("^"));
    let natural = value(Accidental::Natural, tag("="));
    alt((sharpsharp, sharp, flatflat, flat, natural))(input)
}

pub fn octave_count(input: &str) -> IResult<&str, i32> {
    let up_va = tag("'");
    let down_va = tag(",");
    let up_count = map(many1_count(up_va), |n| n as i32);
    let down_count = map(many1_count(down_va), |n| -1 * n as i32);
    map(
        fold_many0(alt((up_count, down_count)), Vec::new, |mut acc, item| {
            acc.push(item);
            acc
        }),
        |v: Vec<i32>| v.iter().sum(),
    )(input)
}

pub fn pitch(input: &str) -> IResult<&str, Note> {
    let mut octave = 4;

    let high = satisfy(|ch| ch >= 'A' && ch <= 'G');
    let low = satisfy(|ch| ch >= 'a' && ch <= 'g');

    let low = map(low, |ch| {
        octave = octave - 1;
        ch.to_ascii_uppercase()
    });

    let some_key = alt((high, low));

    let note = tuple((opt(accidental), some_key, octave_count))(input)?;

    let (rest, (acc, key, mod_octave)) = note;

    Ok((
        rest,
        Note {
            octave: octave + mod_octave,
            key: key,
            accidental: acc,
            length: 1,
        },
    ))
}

const _TUNE1: &'static str = "B>cd BAG";
const _TUNE2: &'static str = "B>cd BAG|FA Ac BA|B>cd BAG|DG GB AG:|";

#[test]
fn parse_pitch() {
    assert_eq!(
        pitch("B"),
        Ok((
            "",
            Note {
                octave: 4,
                key: 'B',
                accidental: None,
                length: 1
            }
        ))
    );
    assert_eq!(
        pitch("^^B"),
        Ok((
            "",
            Note {
                octave: 4,
                key: 'B',
                accidental: Some(Accidental::Sharp2),
                length: 1
            }
        ))
    );
    assert_eq!(
        // octave mark here is: 2 down + 2 up + 1 down
        pitch("=b,,'',"),
        Ok((
            "",
            Note {
                octave: 2,
                key: 'B',
                accidental: Some(Accidental::Natural),
                length: 1
            }
        ))
    );
}
