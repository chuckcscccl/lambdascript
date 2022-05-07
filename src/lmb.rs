// Lambda-Mu-Bang Calculus

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
use std::rc::Rc;
use std::cell::{RefCell};
extern crate fixedstr;
use fixedstr::{str8};
use rustlr::{Tokenizer,RawToken,TerminalToken,StrTokenizer,LBox,LexSource,unbox};
use crate::lmb::Term::*;

extern crate bumpalo;
use bumpalo::Bump;
#[cfg(feature = "collections")]
use bumpalo::collections::{Vec};


pub enum Term<'t>
{
  Var(str8),
  Apply(&'t Term<'t>, &'t Term<'t>),
  Lambda(str8,&'t Term<'t>),
  Mu(str8,&'t Term<'t>),
  Bang(str8,&'t Term<'t>),
  Quest(str8,&'t Term<'t>),  
  Brack(str8,&'t Term<'t>),
  Abort(&'t Term<'t>),
  Break(&'t Term<'t>),
  Stop,
}//Term enum
impl Default for Term<'_>
{ fn default() -> Self {Stop} }


