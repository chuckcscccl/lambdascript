//!  Lambdascript is an educational tool developed by Chuck Liang at
//!  Hofstra University.  See instructions at
//!  <https://cs.hofstra.edu/~cscccl/rustlr_project/lambdascript/README.html>
//!

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_imports)]
extern crate rustlr;
use rustlr::*;
extern crate fixedstr;
use fixedstr::str8;
//mod abstmachine;
//use crate::abstmachine::*;
use chrono;
use chrono::{Datelike,Timelike};
use std::io::Write;
use std::collections::HashMap;

mod untyped;
use untyped::*;
mod untypedparser;

fn main()
{
  println!("Beta-Reducer for Untyped Lambda Calculus, by Chuck Liang.");
  println!("For educational reasons this program may be temporarily disabled during certain time periods");
  let time = chrono::offset::Local::now();

  if time.year()>2022 || time.month()>8 {
    println!("\nThe lifetime of this program has expired. A new version will be released at the appropriate time.");
    return;
  }

  if time.year()==2022 && time.month()==2 && time.day()>=15 && time.day()<=16 {
    println!("\nThis tool is temporarily disabled because of online exams in CSC252DL");
    return;
  }

  let mut parser = untypedparser::make_parser();
  let ref mut defs = HashMap::<str8,Term>::new();
  let ref mut reducer = BetaReducer::new();
  let args:Vec<String> = std::env::args().collect(); // command-line args
  if args.len()>1 {
    let srcfile = &args[1];
    let source = LexSource::new(srcfile).unwrap();
    let mut lexer = LamLexer::new(StrTokenizer::from_source(&source));
    parser.parse(&mut lexer);
    //parser.parse_train(&mut lexer,"src/untypedparser.rs");        
    eval_prog(&parser.exstate,defs,reducer);
    if parser.error_occurred() {
      println!("\nPARSER ERRORS OCCURRED, RESULTS NOT GUARANTEED");
    }
    //return;
  } // source file indicated
  println!("Entering interactive mode, enter 'exit' to quit...");
  loop // will break from within
  {
    print!("<<< ");     let res =std::io::stdout().flush();
    let mut buf = String::new();
    let res2 = std::io::stdin().read_line(&mut buf);

    // process meta-level directives
    let control_d = (4 as char).to_string();
    if buf.len()<3 {continue;}
    else if buf.trim()=="exit" || buf.trim()=="quit" || buf.trim()==&control_d {break;}
    else if buf.trim()=="use lambda" {reducer.setlambda("lambda "); continue;}
    else if buf.trim()=="use lam" {reducer.setlambda("lam "); continue;}
    else if buf.trim()=="use Lam" {reducer.setlambda("Lam "); continue;}
    else if buf.trim()=="use \\" {reducer.setlambda("\\"); continue;}        
    else if buf.trim()=="use greek" || buf.trim()=="use unicode" {reducer.setlambda("\u{03bb}"); continue;}
    else if buf.trim()=="trace off" {reducer.set_trace(1); continue;}
    else if buf.trim()=="trace max" || buf.trim()=="trace on" {reducer.set_trace(5); continue;}
    else if buf.trim()=="trace medium" {reducer.set_trace(2); continue;}        
    
    let mut lexer = LamLexer::new(StrTokenizer::from_str(buf.trim()));

    parser.parse(&mut lexer);
    //parser.parse_train(&mut lexer,"src/untypedparser.rs");    

    eval_prog(&parser.exstate,defs,reducer);
  } // repl 
}//main

