
use std::fs;
use egglog::*;
/*
mod lib;
use lib::*;
mod parser;

use parser::*;
*/

// use lib;
//use parser::*;

fn main() {
    // TODO: better command line grabber
    // Interactve mode?
    let filename = std::env::args().nth(1).expect("no file path given");
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    /* contents.retain(|c| !c.is_whitespace());
    match file(&contents) {
        Err(e) => {
            dbg!(e);
        }
        Ok((rem, file)) => {
            dbg!(rem);
            dbg!(&file);
            run_file(file);
        }
    } */
    dbg!(run(contents));
}
