#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(dead_code)]

extern crate fixedstr;
use fixedstr::{str16};
use rustlr::{unbox, LBox, LexSource, RawToken, StrTokenizer, TerminalToken, Tokenizer};
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};
use crate::untyped::Term;

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
pub enum Lstype {  // lambda script type
  Tvar(u16),       // use indices to mean type variable, t_i
  Tarrow(Box<Lstype>,Box<Lstype>),
  Tconst(str16),
  PI(Box<Lstype>), // generic over all variables.
  Untypable,
}
impl Default for Lstype {
  fn default()->Self { Lstype::Untypable }
}

use Lstype::*;

impl Lstype
{
  // collect all free variables, also find max index
  fn collect_tvars(&self, tvars:&mut BTreeSet<u16>, max:&mut u16) {
    match self {
      Tvar(i) => { tvars.insert(*i); if i>max {*max = *i;} }
      Tarrow(a,b) => {
        a.collect_tvars(tvars,max);
        b.collect_tvars(tvars,max);
      },
      PI(t) => { t.collect_tvars(tvars,max); },
      _ => {},
    }//match
  }//collect_tvars

  fn fresh(&self, symtab:&mut SymbolTable) -> Lstype {
    if let PI(ty) = self {
       let mut freshtype = (&**ty).clone();
       let mut tvars = BTreeSet::new();
       let mut max = 0;
       self.collect_tvars(&mut tvars,&mut max);
       if symtab.index <= max {symtab.set_index(max+1);}
       let mut substs:HashMap<u16,Lstype> = HashMap::new();
       for vi in tvars.iter() {
         let freshi = symtab.newtvar();
         substs.insert(*vi, Tvar(freshi));
       }
       freshtype.mut_subst(&substs);
       freshtype
    }
    else { self.clone() }
  }// fresh copy, instance of Pi type
  
  fn occurs_check(&self, x:u16) -> bool  // true means does not occur
  {  match self {
       Tvar(y) if &x==y => false,
       Untypable => false,
       Tarrow(a,b) => a.occurs_check(x) && b.occurs_check(x),
       _ => true,
     }//match
  }//occurs_check

// non-destructive substitution [t/x] to type (not term)
  fn apply_subst(&self, x:u16, t:&Lstype) -> Lstype
  { match self {
      Tvar(y) if *y==x => t.clone(),
      Tarrow(a,b) => Tarrow(Box::new(a.apply_subst(x,t)),Box::new(b.apply_subst(x,t))),
      _ => self.clone(),
    }//match
  }//apply_subst
  fn mut_subst(&mut self, substs:&HashMap<u16,Lstype> ) {
    match self {
      Tvar(y) if substs.contains_key(y) => { *self = substs[y].clone(); },
      Tarrow(a,b) => {
         a.mut_subst(substs);
         b.mut_subst(substs);
      }
      _ => {},
    }//match
  }//mut_subst

  pub fn format(&self) -> String {
    match self {
      Tvar(index) => {
        if index<&26 {format!("{}",((*index as u8)+97) as char)}
        else {format!("t_{}",index-25)}
      },
      Tconst(s) => s.to_string(),
      Untypable => String::from("UNTYPABLE"),
      Tarrow(a,b) => {
        let (fa,fb) = (a.format(), b.format());
        if fa.len()<5 { // no () needed
          format!("{} -> {}", &fa, &fb)
        }
        else {
          format!("({}) -> {}",&fa, &fb)
        }
      },
      PI(t) => format!("{}({})", PISYM, t.format())
    }//match
  }//format

}//impl Lstype

const PISYM: &'static str = "\u{03a0}";

// unification algorithm
type Equations = Vec<(Lstype,Lstype)>;

//returns substitution map for unifier on success
fn unify_types(equations:&mut Equations) -> Option<HashMap<u16,Lstype>>
{
  let mut unifier = HashMap::new();
  let mut failure = false;
  let mut eqi = 0; //indexes equations
  while eqi < equations.len()  // break when failure detected
  {
    let mut neweqs:BTreeMap<usize,(Lstype,Lstype)> = BTreeMap::new();
    let (ref s,ref t) = equations[eqi];
    if (s==t) { eqi+=1; continue; }
    match (s,t) {
      (Tvar(x),u) | (u,Tvar(x)) if /* u!=&Tvar(*x) && */ u.occurs_check(*x) => {
        for i in 0..equations.len() {
          if i==eqi {continue;}
          let (ref ta,ref tb) = equations[i];
          let ta2 = ta.apply_subst(*x,u);
          let tb2 = tb.apply_subst(*x,u);
          //equations[i] = (ta2,tb2);  // mutation!
          neweqs.insert(i,(ta2,tb2));
        }//for
      },
      (Tarrow(a1,a2),Tarrow(b1,b2)) => {
        let elen = equations.len();
        neweqs.insert(elen,((&**a1).clone(),(&**b1).clone())); // should use bump
        neweqs.insert(elen+1,((&**a2).clone(),(&**b2).clone()));
      },
      (Tconst(a),Tconst(b)) if a==b => {}, // do nothing
      _ => {failure=true; break; }
    }//match
    let originalen = equations.len();
    for (i,(a,b)) in neweqs {
      if i<originalen { equations[i] = (a,b); }
      else { equations.push((a,b));}
    }
    eqi += 1;
  }//while eqi<equations.len()
  // construct unifier
  eqi = equations.len();
  while eqi>0 
  {
     eqi -= 1;
     match &equations[eqi] {
       (Tvar(x), u) | (u,Tvar(x)) if !unifier.contains_key(x) => {
         unifier.insert(*x,u.clone());
       },
       _ => (),
     }//match
  }// while eqi>0
  //println!("{} (failed?) unifier: {:?}",failure,&unifier);
  if failure {None} else {Some(unifier)}
}//unify_types


// type inference
///// need symbol table to infer types.
// can the symbol table just be a stack? or stack of frames?

#[derive(Default,Debug)]
pub struct SymbolTable {
  stack : Vec<(str16,Lstype)>,
  index : u16, // for naming type variables
}
impl SymbolTable {
  fn newtvar(&mut self) -> u16 {
    self.index += 1;
    self.index-1
  }
  pub fn reset_index(&mut self) {
    self.index=0;
  }
  fn set_index(&mut self, i:u16) {
    self.index = i;
  }
  pub fn add(&mut self, varname:str16, ty:Lstype) {
    self.stack.push((varname,ty));
  }
  fn lookup(&self, x:&str16) -> Option<usize> { // returns index found
    let mut i = self.stack.len();
    while i>0 {
      let entry = &self.stack[i-1];
      if x==&entry.0 && &entry.1 != &Untypable {return Some(i-1);} 
      i-=1;
    }//while
    None
  }//lookup
  fn apply_unifier(&mut self, unifier:&HashMap<u16,Lstype>) {
    let mut i = self.stack.len();
    while i > 0 {
      let entry = &self.stack[i-1];
      let mut newtype = entry.1.clone();
      if &newtype!=&Untypable {
        for (k,v) in unifier.iter() {
          newtype = newtype.apply_subst(*k,v);
        }
        self.stack[i-1].1 = newtype;
      }
      i -= 1;
    }//while
  }//apply_unifier
}// impl SymbolTable


impl Term
{
  pub fn type_infer(&self, symtab:&mut SymbolTable) -> Lstype
  {
    let stack_bp = symtab.stack.len();
    let answer = self.infer_type(symtab);
    symtab.stack.truncate(stack_bp);
    answer
  }

  fn infer_type(&self, symtab:&mut SymbolTable) -> Lstype
  { use Term::*;
    let mut answer = Untypable;
    match &self {
      Var(x) => {
        if let Some(xi) = symtab.lookup(x) {
            answer = symtab.stack[xi].1.clone();
            if let PI(_) = answer {
              answer = answer.fresh(symtab);
            }
        }
        else {
            let ti = symtab.newtvar();
            symtab.stack.push((*x,Tvar(ti)));
            answer = Tvar(ti);        
        }
      },
      Abs(x,m) => {
        let ti = symtab.newtvar();
        let xpos = symtab.stack.len();
        symtab.stack.push((*x,Tvar(ti)));
        let tm = m.infer_type(symtab);
        //if &tm != &Untypable {
          let tx = Var(*x).infer_type(symtab); // type may have changed
          //if &tx != &Untypable {
            symtab.stack[xpos].1 = Untypable; // mark position as invalid
            let mut i = symtab.stack.len();
            while i>0 && &symtab.stack[i-1].1==&Untypable {
              symtab.stack.pop();
              i -= 1;
            }
            answer = Tarrow(bx(tx),bx(tm));
          //}
        //}
      },
      App(s,t) => {
        let ts = s.infer_type(symtab);
        // unify with new type
        let ds = symtab.newtvar();
        let cs = symtab.newtvar();
        let ts2 = Tarrow(bx(Tvar(ds)),bx(Tvar(cs)));
        let tt = t.infer_type(symtab);
        //println!("type of tt: {:?}", &tt);
        //if &tt==&Untypable { return answer; }
        let mut type_equations = vec![(ts,ts2), (tt,Tvar(ds))];
        let mut unifyresult = unify_types(&mut type_equations);
        if let Some(mut unifier) = unifyresult {
          symtab.apply_unifier(&unifier);
          if let Some(ttype) = unifier.remove(&cs) {
            answer = ttype;
          }
          else {
            answer = Tvar(cs); // not in support of unifier - stays same
          }
        }
        // but how are types in the symbol table affected?
        // types ts, tt should also be affected by the unification.
        // must apply the unifier to the symtab.stack
      },
      Const(_) => { answer = Tconst(str16::from("int")); },
      Def(_,n,t) => {
        answer = t.infer_type(symtab);
        symtab.stack.push((*n,answer.clone()));
      },
      Weak(t) | CBV(t) => { answer = t.infer_type(symtab); },
      Seq(ts) => {
        for t in ts { answer = t.infer_type(symtab); }
      },
      _ => {},
    }// match
    answer
  }//infer_type
}//impl Term


fn bx<T>(x:T) -> Box<T> { Box::new(x) }
