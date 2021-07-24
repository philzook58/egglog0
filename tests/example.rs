//use lib::*;
use eggdl::run;
use std::fs;

#[test]
fn it_adds_two() {
    let filename = "tests/example.pl";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    dbg!(run(contents));
}
