use std::env;

use abcrab::parser::pitch;

fn main() {
    let aa: Vec<String> = env::args().collect();
    let (_, note) = pitch(aa[1].as_str()).unwrap();
    println!("{}", note);
}
