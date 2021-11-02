use egglog::*;
use std::fs;
/*
mod lib;
use lib::*;
mod parser;

use parser::*;
*/

// use lib;
//use parser::*;

/*
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn repl() {
    let mut rl = Editor::<()>::new();
    let mut prog = Program::default();
    println!(
        "\
    Egglog - Philip Zucker <philzook58@gmail.com> 2021\n\
    Type \":- help.\" for more information.\n\
    "
    );
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match parse_file(line) {
                    Ok(entries) => {
                        for entry in entries {
                            process_entry_prog(&mut prog, entry);
                        }
                    }
                    Err(e) => {
                        println!("Error: Could not parse. {}", e);
                    }
                }
                // println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
*/
fn repl() {
    println!("Sorry. REPL currently disabled because wasm was being weird. Please select a file to run as an argument.")
}

use clap::Clap;
fn main() {
    // TODO: better command line grabber
    // Interactve mode?
    let opts: Opts = Opts::parse();
    let filename = &opts.filename; // td::env::args().nth(1).expect("no file path given");
    match filename {
        Some(filename) => {
            let contents =
                fs::read_to_string(filename).expect("Something went wrong reading the file");
            match run(contents, &opts) {
                Ok(res) => println!("Results : \n{}", res),
                Err(err) => println!("Error : \n {}", err),
            }
        }
        None => repl(),
    }
}
