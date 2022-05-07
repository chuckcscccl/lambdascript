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
use crate::exsubs::Sterm::*;
use crate::exsubs::Eterm::*;

extern crate bumpalo;
use bumpalo::Bump;
#[cfg(feature = "collections")]
use bumpalo::collections::{Vec};


/* Lambda terms in annotated suspension notation
  Categories:
  Terms
  Environments
  Environment terms
  Annotation (open/closed )
*/

// bool for open/closed: true for closed, false for open
// Susp form designed to maximize pattern matching

// Copy,Clone trait only clones outer form, since subterms are all references.
#[derive(Copy,Clone)]
pub enum Sterm<'b> // term, suspension term
{
  Constant(i64),
  Lvar(str8),      // logic variable
  Ind(u32),
  Appl(&'b Sterm<'b>,&'b Sterm<'b>,bool), 
  Abst(bool,&'b Sterm<'b>),
  Susp(&'b Sterm<'b>,u32,u32,&'b [RefCell<Eterm<'b>>],bool),
  Nothing,  // used for default
}//Sterm
/*
impl<'b> Sterm<'b>
{
  pub fn app(arena:&'b Bump,
}//impl Sterm
*/

#[derive(Copy,Clone)]
pub enum Eterm<'b>  // environment term
{
   Atlev(u32),
   TLpair(&'b Sterm<'b>,u32),
}
impl<'b> Eterm<'b>
{
  pub fn isat(&self,lev:u32) -> bool
  {
      match self {
         Atlev(n) if *n==lev => true,
         _ => false,
      }
  }//isat
//  pub fn ispair(&self)
}//impl Eterm

/*
enum Envlist<'b>
{
  Nilenv,
  Cons(Eterm<'b>,Rc<Envlist<'b>>),
}
pub struct Environment<'b>
{
   env:Envlist<'b>,
   refs:Vec<&'b Eterm<'b>>,  // for quick lookup (reversed)
}
impl<'b> Environment<'b>
{
   pub fn push(&mut self, t:Eterm<'b>)
   {
      let tail = Rc::new(self.env);
      self.refs.push(&t);
      self.env = Envlist::Cons(t,tail);
   }//
}//impl Environment
*/

// basic consuming rewrite step
pub fn rewrite<'b>(mut term:Sterm<'b>, arena: &'b Bump) -> Sterm<'b>
{
  match term {
    Appl(Abst(u,Susp(t1,olp1,nlp1,env@[..,et],false)),t2,v)
    if env[env.len()-1].borrow().isat(nlp1-1) =>  { // Beta'_s case   
      et.replace(TLpair(t2,nlp1-1));
      Susp(t1,*olp1,nlp1-1,env,v)
    },
    Appl(Abst(u,t1),t2,v) => {   // Beta_s case
      Susp(t1,1,0,arena.alloc(vec![RefCell::new(TLpair(t2,0))]),v)
    },
    Susp(t,0,0,[],u) => *t, // r12  (surface clone)
    Susp(Constant(c),ol,nl,e,u) => Constant(*c),     // case r1
    Susp(Lvar(s),ol,nl,e,u) => Lvar(*s),     // case r2
    Susp(Ind(i),ol,nl,e,u) if *i>ol => Ind(*i-ol+nl),  //r3
    Susp(Ind(i),ol,nl,e,u) => { // r4, r5
      match &*e[e.len()-1-(*i as usize)].borrow() {
         Atlev(l) => Ind(nl-l),    // r4
         TLpair(t,l) => Susp(t,0,nl-l,arena.alloc(vec![]),u),
      }//submatch
    },
    Susp(Appl(t1,t2,true),_,_,_,_) => Appl(t1,t2,true),  // r8
    Susp(Appl(t1,t2,u),ol,nl,e,v) => { // r6
      Appl(arena.alloc(Susp(t1,ol,nl,e,v)),
           arena.alloc(Susp(t2,ol,nl,e,v)),v)
    },
    Susp(Abst(true,t),_,_,_,_) => Abst(true,t),  // r9
    Susp(Abst(u,t),ol,nl,e,v) => {      // r7
      //e.push(Atlev(nl));  // won't compile!
      let me = arena.alloc(Vec::with_capacity(e.len()+1));
      for i in 0..e.len()
      {
        let temp = e[i].replace(Atlev(99999));  // better than cloning
        me.push(RefCell::new(temp));  
      }
      me.push(RefCell::new(Atlev(nl)));
      Abst(v,arena.alloc(Susp(t,ol+1,nl+1,me,false)))
    },
    Susp(Susp(t,ol,nl,e,true),_,_,_,_) => Susp(t,*ol,*nl,e,true),  // r10
    Susp(Susp(t,ol,nl,e,false),0,nlp,[],open) => // r11
      Susp(t,*ol,nl+nlp,e,false),
    _ => term,  // same term returned by default
  }//match
}//rewrite
