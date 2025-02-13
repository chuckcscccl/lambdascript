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
use crate::untyped::{Term,BetaReducer};

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

  // special type (a -> a) -> a for fixedpoint operator
  pub fn fixpttype() -> Lstype {
    let ara = Tarrow(Box::new(Tvar(1)), Box::new(Tvar(1)));
    Tarrow(Box::new(ara),Box::new(Tvar(1)))
  }

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

  fn apply_unifier(&self, unifier:&HashMap<u16,Lstype> ) -> Lstype {
    match self {
      Tvar(y) if unifier.contains_key(y) => unifier[y].clone(),
      Tarrow(a,b) => {
         let ta = a.apply_unifier(unifier);
         let tb = b.apply_unifier(unifier);
         Tarrow(Box::new(ta), Box::new(tb))
      }
      _ => self.clone(),
    }//match    
  }

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
      PI(t) => {
        match &**t {
          Tconst(s) => s.to_string(),
          Untypable => String::from("UNTYPABLE"),
          _ => format!("{}({})", PISYM, t.format())
        }//submatch for PI case
      }
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
  bp    : usize, // stack base index (for global defs)
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
  pub fn add_def(&mut self, varname:str16, ty:Lstype) {
    self.stack.push((varname,ty));
    self.bp = self.stack.len();
  }
  pub fn insert_def(&mut self, varname:str16, ty:Lstype) {
    self.stack.insert(0,(varname,ty));
    self.bp += 1;
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
    while i > self.bp {
      let entry = &self.stack[i-1];
      let mut newtype = entry.1.clone();
      if &newtype!=&Untypable {
        for (k,v) in unifier.iter() {
          newtype = newtype.apply_subst(*k,v);
        }
//println!("type of {} updated to {}",self.stack[i-1].0, newtype.format());
        self.stack[i-1].1 = newtype;
      }
      i -= 1;
    }//while
  }//apply_unifier
}// impl SymbolTable


impl Term
{
  pub fn type_infer(&self, reducer:&mut BetaReducer) -> Lstype
  {
    let stack_len = reducer.symtab.stack.len();
    let answer = self.infer_type(reducer);
    reducer.symtab.stack.truncate(stack_len);
    answer
  } // BetaReducer also contains the type symbol table + global definitions

  fn infer_type(&self, reducer:&mut BetaReducer) -> Lstype
  { use Term::*;
    let mut answer = Untypable;
    match &self {
      Var(x) => {
        if let Some(xi) = reducer.symtab.lookup(x) {
            answer = reducer.symtab.stack[xi].1.clone();
            if let PI(_) = answer {
              answer = answer.fresh(&mut reducer.symtab);
            }
        }
        else if let Some(def) = reducer.defs.get(x) { // global definition
          let defclone = def.clone();
          answer = PI(bx(defclone.infer_type(reducer)));
          reducer.symtab.insert_def(*x,answer.clone());
          answer = answer.fresh(&mut reducer.symtab);
          // this case applies only if switching from untyped to typed mode
        }
        else { // free variable - undefined
            //let ti = symtab.newtvar();
            //symtab.stack.push((*x,Tvar(ti)));
            //answer = Tvar(ti);        
            
            println!("In the typed mode, the undefined free variable {} cannot be typed",x);
        }
      },

////////// try global unification set of equations?

      Abs(x,m) => {
        let ti = reducer.symtab.newtvar();
        let xpos = reducer.symtab.stack.len();
        reducer.symtab.stack.push((*x,Tvar(ti)));
        let tm = m.infer_type(reducer);
        let tx = Var(*x).infer_type(reducer); // type may have changed
        reducer.symtab.stack[xpos].1 = Untypable; //mark position as invalid
        let mut i = reducer.symtab.stack.len();
        while i>0 && &reducer.symtab.stack[i-1].1==&Untypable {
          reducer.symtab.stack.pop();
          i -= 1;
        }
        answer = Tarrow(bx(tx),bx(tm));
      },
      App(s,t) => {
        let tt = t.infer_type(reducer);
        let ts = s.infer_type(reducer);
        //let ds = reducer.symtab.newtvar();        
        let cs = reducer.symtab.newtvar();
        let ts2 = Tarrow(bx(tt),bx(Tvar(cs)));


        //let mut type_equations = vec![(ts,ts2), (tt,Tvar(ds))];
        let mut type_equations = vec![(ts,ts2)];        
        let mut unifyresult = unify_types(&mut type_equations);
        if let Some(mut unifier) = unifyresult {
          reducer.symtab.apply_unifier(&unifier);
          if let Some(ttype) = unifier.remove(&cs) {
            answer = ttype;
          }
          else {
            answer = Tvar(cs); // not in support of unifier - stays same
          }
        }
        // but how are types in the symbol table affected?
        // types ts, tt should also be affected by the unification.
        // must apply the unifier to the reducer.symtab.stack
      },
      Const(_) => { answer = Tconst(str16::from("int")); },
      Def(_,n,t) => {
        answer = t.infer_type(reducer);
        reducer.symtab.stack.push((*n,answer.clone()));
        reducer.symtab.bp = reducer.symtab.stack.len();
      },
      Weak(t) | CBV(t) => { answer = t.infer_type(reducer); },
      Seq(ts) => {
        for t in ts { answer = t.infer_type(reducer); }
      },
      _ => {},
    }// match
    answer
  }//infer_type
}//impl Term


fn bx<T>(x:T) -> Box<T> { Box::new(x) }

fn print_equations(e:&[(Lstype,Lstype)]) {
   println!("equations: --------");
   for (a,b) in e {
     println!("{}  =  {}", a.format(), b.format());
   }
}

fn print_unifier(u:&HashMap<u16,Lstype>) {
   println!("unifier:------");
   for (a,b) in u.iter() {
     println!("{} -> {}", Tvar(*a).format(),b.format());
   }
}