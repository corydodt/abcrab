use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, satisfy},
    combinator::opt,
    combinator::{map, map_res, value},
    multi::fold_many0,
    multi::many1_count,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{Accidental, Length, Note, NoteShape};

fn accidental(input: &str) -> IResult<&str, Accidental> {
    let flatflat = value(Accidental::Flat2, tag("__"));
    let flat = value(Accidental::Flat, tag("_"));
    let sharpsharp = value(Accidental::Sharp2, tag("^^"));
    let sharp = value(Accidental::Sharp, tag("^"));
    let natural = value(Accidental::Natural, tag("="));
    alt((sharpsharp, sharp, flatflat, flat, natural))(input)
}

fn octave_count(input: &str) -> IResult<&str, i32> {
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

    let low = satisfy(|ch| ('A'..='G').contains(&ch));
    let high = satisfy(|ch| ('a'..='g').contains(&ch));

    let high = map(high, |ch| {
        octave += 1;
        ch.to_ascii_uppercase()
    });

    let some_key = alt((high, low));

    let (rest, (acc, key, mod_octave)) = tuple((opt(accidental), some_key, octave_count))(input)?;

    Ok((
        rest,
        Note {
            octave: octave + mod_octave,
            key,
            accidental: acc,
            length: Length {
                note_shape: NoteShape::Eighth,
                dot: 0,
            },
        },
    ))
}

pub fn length(input: &str) -> IResult<&str, Length> {
    let num1 = map_res(digit1, |s: &str| s.parse::<i32>());
    let num2 = map_res(digit1, |s: &str| s.parse::<i32>());
    let num3 = map_res(digit1, |s: &str| s.parse::<i32>());

    let num_num = tuple((num1, preceded(tag("/"), num2)));
    let slash_num = map(preceded(tag("/"), num3), |n| (1, n));
    let base2: i32 = 2;
    let slash_only = map(many1_count(tag("/")), |n| (1, base2.pow(n as u32)));
    let (rest, (numer, denom)) = alt((num_num, slash_num, slash_only))(input)?;

    Ok((rest, Length::new((numer, denom)).unwrap()))
}

const _TUNE1: &str = "B>cd BAG";
const _TUNE2: &str = "B>cd BAG|FA Ac BA|B>cd BAG|DG GB AG:|";

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
                length: Length {
                    note_shape: NoteShape::Eighth,
                    dot: 0
                },
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
                length: Length {
                    note_shape: NoteShape::Eighth,
                    dot: 0
                }
            }
        ))
    );
    assert_eq!(
        // octave mark here is: 2 down + 2 up + 1 down, and lowercase starts +1
        pitch("=b,,'',"),
        Ok((
            "",
            Note {
                octave: 4,
                key: 'B',
                accidental: Some(Accidental::Natural),
                length: Length {
                    note_shape: NoteShape::Eighth,
                    dot: 0
                }
            }
        ))
    );
}

#[test]
fn parse_length() {
    assert_eq!(
        length("7/16"),
        Ok((
            "",
            Length {
                note_shape: NoteShape::Quarter,
                dot: 2
            }
        ))
    );
    assert_eq!(
        length("/2"),
        Ok((
            "",
            Length {
                note_shape: NoteShape::Half,
                dot: 0
            }
        ))
    );
    assert_eq!(
        length("//"),
        Ok((
            "",
            Length {
                note_shape: NoteShape::Quarter,
                dot: 0
            }
        ))
    );
}
