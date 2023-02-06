#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_imports)]
extern crate rustlr;
use rustlr::*;
extern crate fixedstr;
use fixedstr::str16;
//mod abstmachine;
//use crate::abstmachine::*;
//use chrono;
//use chrono::{Datelike, Timelike};
use std::collections::HashMap;
use std::io::Write;

mod untyped;
use untyped::*;
mod untypedparser;
use untypedparser::*;

fn main() {
    println!("Beta-Reducer for Untyped Lambda Calculus, by Chuck Liang.");
    println!("For educational reasons this program may be temporarily disabled during certain time periods");
    if !lambda_formal() {
        return;
    }
    let mut parser = make_parser();
    let ref mut defs = HashMap::<str16, Term>::new();
    let args: Vec<String> = std::env::args().collect(); // command-line args
    if args.len() > 1 {
        let srcfile = &args[1];
        let source = LexSource::new(srcfile).unwrap();
        //    let mut lexer = LamLexer::new(StrTokenizer::from_source(&source));
        let mut lexer = untypedlexer::from_source(&source);
        parser.parse(&mut lexer);
        //parser.parse_train(&mut lexer,"src/untypedparser.rs");
        eval_prog(&parser.exstate, defs);
        if parser.error_occurred() {
            println!("\nPARSER ERRORS OCCURRED, RESULTS NOT GUARANTEED");
        }
        //return;
    } // source file indicated
    println!("Entering interactive mode, enter 'exit' to quit...");
    loop
    // will break from within
    {
        print!("<<< ");
        let res = std::io::stdout().flush();
        let mut buf = String::new();
        let res2 = std::io::stdin().read_line(&mut buf);
        let bln = buf.len();
        let buftrim = buf.trim();
        if bln < 3 {
            continue;
        } else if buftrim == "exit" || buftrim == "quit" {
            break;
        }

        if !buftrim.ends_with(';') {
            buf = format!("{};", buftrim);
        }

        let mut lexer = untypedlexer::from_str(buf.trim());
        parser.exstate.clear();
        parser.parse(&mut lexer);
        //parser.parse_train(&mut lexer,"src/untypedparser.rs");
        //println!("exstate: {:?}",&parser.exstate);

        eval_prog(&parser.exstate, defs);
    } // repl
} //main
