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
use clap::{AppSettings, Clap};

/// A Prolog-like theorem prover based on Egg
#[derive(Clap)]
#[clap(version = "0.01", author = "Philip Zucker <philzook58@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Path of Egglog file to run
    filename: Option<String>,
}

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn repl() {
    let mut rl = Editor::<()>::new();
    let mut env = Env::default();
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
                            process_entry(&mut env, entry);
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

fn main() {
    // TODO: better command line grabber
    // Interactve mode?
    let opts: Opts = Opts::parse();
    let filename = opts.filename; // td::env::args().nth(1).expect("no file path given");
    match filename {
        Some(filename) => {
            let contents =
                fs::read_to_string(filename).expect("Something went wrong reading the file");
            match run(contents) {
                Ok(res) => println!("Results : \n{}", res),
                Err(err) => println!("Error : \n {}", err),
            }
        }
        None => repl(),
    }
}
