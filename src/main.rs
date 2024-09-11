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

mod typed;
use typed::*;

fn main() {
    println!("Beta-Reducer for Lambda Calculus.\nCopyright (c) 2022 Chuck Liang. Free to use under MIT license.");
    //println!("For educational reasons this program may be temporarily disabled during certain time periods");
    //if !lambda_formal() {    return;    }
    let mut parser = make_parser();
    let mut reducer = BetaReducer::new();
    let args: Vec<String> = std::env::args().collect(); // command-line args
    let mut srcindex = 1;
    if args.len()>1 && &args[1]=="typed" {
      reducer.settyped(true);
      srcindex = 2;
    }
    else if args.len()>1 && &args[1]=="untyped" {
      reducer.settyped(false);
      srcindex = 2;    
    }
    if args.len() > srcindex {
        let srcfile = &args[srcindex];
        let source = LexSource::new(srcfile).unwrap();
        //    let mut lexer = LamLexer::new(StrTokenizer::from_source(&source));
        let mut lexer = untypedlexer::from_source(&source);
        parser.parse(&mut lexer);
        //parser.parse_train(&mut lexer,"src/untypedparser.rs");
        eval_prog(&parser.exstate, &mut reducer);
        if parser.error_occurred() {
            println!("\nPARSING ERRORS OCCURRED, RESULTS NOT GUARANTEED");
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
        if bln < 2 {
            continue;
        } else if buftrim == "exit" || buftrim == "quit" {
            break;
        }
        else if buftrim == "typed" {
          reducer.settyped(true);
          println!("TERMS DEFINED IN THE UNTYPED MODE WILL NOW BE TYPE-CHECKED BEFORE EVALUATION");
          continue;
        }
        else if buftrim == "untyped" {
          reducer.settyped(false);
          continue;
        }
   else if buftrim=="use lambda" {reducer.setlambda("lambda "); continue;}
    else if buftrim=="use lam" {reducer.setlambda("lam "); continue;}
    else if buftrim=="use Lam" {reducer.setlambda("Lam "); continue;}
    else if buftrim=="use \\" {reducer.setlambda("\\"); continue;}        
    else if buftrim=="use greek" || buftrim=="use unicode" {reducer.setlambda("\u{03bb}"); continue;}
    else if buftrim=="trace off" {reducer.set_trace(1); continue;}
    else if buftrim=="trace max" || buftrim=="trace on" {reducer.set_trace(5); continue;}
    else if buftrim=="trace medium" {reducer.set_trace(2); continue;}
    
        if !buftrim.ends_with(';') {
            buf = format!("{};", buftrim);
        }

        let mut lexer = untypedlexer::from_str(buf.trim());
        parser.exstate.clear();
        parser.parse(&mut lexer);
        //parser.parse_train(&mut lexer,"src/untypedparser.rs");
        //println!("exstate: {:?}",&parser.exstate);

        eval_prog(&parser.exstate, &mut reducer);
    } // repl
} //main
