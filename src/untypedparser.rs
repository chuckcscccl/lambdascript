//Parser generated by rustlr for grammar untyped

#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(irrefutable_let_patterns)]
#![allow(unreachable_patterns)]
extern crate rustlr;
use rustlr::{Tokenizer,TerminalToken,ZCParser,ZCRProduction,Stateaction,decode_action};
use rustlr::{StrTokenizer,RawToken,LexSource};
use std::collections::{HashMap,HashSet};
use rustlr::{LBox,unbox};
use crate::untyped::*;
use crate::untyped::Term::*;
use fixedstr::str8;

static SYMBOLS:[&'static str;28] = ["lambda","lam","Lam","(",")","[","]","DOT","let","=","in","define","lazy","weak","CBV","Liang",";","INTEGER","ID","T","F","Fs","TOP","Vars","LAMSYM","Ts","START","EOF"];

static TABLE:[u64;214] = [107374444545,73014771712,85899739137,94489411585,81605033985,55835295744,8590852096,4295753728,103080263681,77309607936,60130394112,12885360640,90194837505,34359803904,983040,47245230080,281552287236096,563018674077696,844437815296002,844493649870850,844442110263298,844497944838146,844502239805442,844467880067074,1126015870959619,1125994397368321,1125990101680129,1125985806581761,1125960037236736,1125955742138368,1125972921614336,1125908497694720,1125947152072704,1125912792203264,1125977216450560,1125904202596352,1125981511876609,1126002987106305,1125899907825664,1125934266646528,1407417833553922,1407443603357698,1407452193292290,1407387768782850,1407392063750146,1407447898324994,1688862745296898,1688867040264194,1688927169806338,1688918579871746,1688922874839042,1688892810067970,1970380672270336,1970333427826688,1970359196778496,1970397851746304,1970406442663937,1970410736713729,1970337722335232,1970324837957632,1970427917238273,1970402146582528,1970329132728320,1970415031812097,1970384967368704,2251872828456960,2251868533555202,2251885714407425,2251842763751426,2251877123293184,2251816993947650,2251812699045888,2533352101314560,2533326331445248,2814818487631874,3096297758588928,3096302053425152,3096310644736001,3096237629177856,3377777031118850,3659247712010240,3659260598222849,3659252006846464,3659187582599168,3940726984605698,4222201961185282,4503698413322241,4503676938551296,4785113260621824,5066665544908802,5066609710333954,5066558170726402,5066626890203138,5066553875759106,5066605415366658,5066562465693698,5066583940530178,5066549580791810,5066622595235842,5066596825432066,5348093278879744,5629516716048384,5910987396022274,5911051820531714,5911017460793346,5911043230597122,5910991690989570,5911047525564418,6192526799077376,6473963121147904,6755442391318530,6755468161122306,6755416621514754,7036917367963650,7036943137767426,7036891598159874,7318426706051072,7318379461476352,7599854436679682,7599901681319938,7881389542735873,7881299348881408,7881333707702272,7881307938750464,7881355183194112,7881372362670080,7881402428162049,7881380954570753,7881359478292480,7881385247637505,7881312233259008,7881303643652096,7881376657506304,8162830159249410,8162851634085890,8162821569314818,8162890288791554,8162847339118594,8162787209576450,8162774324674562,8162778619641858,8162834454216706,8162782914609154,8162808684412930,8444322316222466,8444326611189762,8444262186680322,8444318021255170,8444292251451394,8444266481647618,8725762935095296,9007280861544449,9007289449578497,9007259385135104,9007302335004673,9007203550494720,9007272269512704,9007285154480129,9007233614544896,9007212140101632,9007199255724032,9007276564348928,9007255090036736,9007207845593088,9288704297009154,9288751541649410,9570252288425985,9570205043458048,9570230815031297,9570162093522944,9570239402999809,9570183567966208,9570153503916032,9570209338556416,9570226517770240,9570222222934016,9570149209145344,9570235107901441,9570157799014400,9851667137101824,10133159291977728,10133107752435712,10133154996879360,10133180768583681,10133112046944256,10133202241847297,10133099162566656,10133172176355328,10133133521387520,10133176471191552,10133103457337344,10133189356421121,10133185061322753,10414642858688514,10696092065333250,10696066295529474,10696117835137026,10977627171979265,10977609991454721,10977597106487296,10977614286553089,10977532682567680,10977584222109696,10977601401323520,10977536977076224,10977579927011328,10977558451519488,10977528387469312,10977605698781185,10977524092698624,11259067788886018,11540516995661826,11540491225858050,11540542765465602,];

pub fn make_parser() -> ZCParser<Term,Vec<LBox<Term>>>
{
 let mut parser1:ZCParser<Term,Vec<LBox<Term>>> = ZCParser::new(21,42);
 let mut rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("start");
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("Ts");
 rule.Ruleaction = |parser|{ let mut _item1_ = parser.popstack(); let mut x = parser.popstack();  parser.exstate.push(x.lbox()); Nothing };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("Ts");
 rule.Ruleaction = |parser|{ let mut _item2_ = parser.popstack(); let mut x = parser.popstack(); let mut _item0_ = parser.popstack();  parser.exstate.push(x.lbox()); Nothing };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("Fs");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); 
  if let (a,)=(_item0_.value,) {  a }  else {parser.bad_pattern("(a,)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("Fs");
 rule.Ruleaction = |parser|{ let mut b = parser.popstack(); let mut a = parser.popstack();  App(a.lbox(), b.lbox()) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("F");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); 
  if let ((x),)=(_item0_.value,) {  x } /* var */  else {parser.bad_pattern("((x),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("F");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); 
  if let ((x),)=(_item0_.value,) {  x } /* const*/  else {parser.bad_pattern("((x),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("T");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); 
  if let (a,)=(_item0_.value,) {  a }  else {parser.bad_pattern("(a,)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("F");
 rule.Ruleaction = |parser|{ let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let (a,)=(_item1_.value,) {  a }  else {parser.bad_pattern("(a,)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("T");
 rule.Ruleaction = |parser|{ let mut x = parser.popstack(); let mut _item0_ = parser.popstack();  CBV(x.lbox()) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("T");
 rule.Ruleaction = |parser|{ let mut x = parser.popstack(); let mut _item0_ = parser.popstack();  Weak(x.lbox()) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("T");
 rule.Ruleaction = |parser|{ let mut b = parser.popstack(); let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let (Seq(mut vs),)=(_item1_.value,) { 
  let mut t = b.value;
  while vs.len()>0 {
    t = Abs(getvar(&unbox!(vs.pop().unwrap())),parser.lbx(0,t));
  }
  return t; }  else {parser.bad_pattern("(Seq(mut vs),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("Vars");
 rule.Ruleaction = |parser|{ let mut x = parser.popstack();  Seq(vec![x.lbox()]) };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("Vars");
 rule.Ruleaction = |parser|{ let mut y = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let (Seq(mut vs),)=(_item0_.value,) {  vs.push(y.lbox()); Seq(vs) }  else {parser.bad_pattern("(Seq(mut vs),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("T");
 rule.Ruleaction = |parser|{ let mut b = parser.popstack(); let mut _item4_ = parser.popstack(); let mut v = parser.popstack(); let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let (Var(x),)=(_item1_.value,) {  App(parser.lbx(0,Abs(x,b.lbox())), v.lbox()) }  else {parser.bad_pattern("(Var(x),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("TOP");
 rule.Ruleaction = |parser|{ let mut v = parser.popstack(); let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let (Var(x),)=(_item1_.value,) { 
  let nv = Def(true,x,v.lbox());
  //parser.exstate.push(parser.lbx(0,nv));
  nv 
 }  else {parser.bad_pattern("(Var(x),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("TOP");
 rule.Ruleaction = |parser|{ let mut v = parser.popstack(); let mut _item3_ = parser.popstack(); let mut _item2_ = parser.popstack(); let mut _item1_ = parser.popstack(); let mut _item0_ = parser.popstack(); 
  if let (Var(x),)=(_item2_.value,) { 
  let nv = Def(false,x,v.lbox());
  nv 
 }  else {parser.bad_pattern("(Var(x),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("TOP");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); 
  if let ((x),)=(_item0_.value,) {  x }  else {parser.bad_pattern("((x),)")} };
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("LAMSYM");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); <Term>::default()};
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("LAMSYM");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); <Term>::default()};
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("LAMSYM");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); <Term>::default()};
 parser1.Rules.push(rule);
 rule = ZCRProduction::<Term,Vec<LBox<Term>>>::new_skeleton("START");
 rule.Ruleaction = |parser|{ let mut _item0_ = parser.popstack(); <Term>::default()};
 parser1.Rules.push(rule);
 parser1.Errsym = "";
 parser1.resynch.insert(";");

 for i in 0..214 {
   let symi = ((TABLE[i] & 0x0000ffff00000000) >> 32) as usize;
   let sti = ((TABLE[i] & 0xffff000000000000) >> 48) as usize;
   parser1.RSM[sti].insert(SYMBOLS[symi],decode_action(TABLE[i]));
 }

 for s in SYMBOLS { parser1.Symset.insert(s); }

 load_extras(&mut parser1);
 return parser1;
} //make_parser


// Lexical Scanner using RawToken and StrTokenizer
pub struct untypedlexer<'t> {
   stk: StrTokenizer<'t>,
   keywords: HashSet<&'static str>,
}
impl<'t> untypedlexer<'t> 
{
  pub fn from_str(s:&'t str) -> untypedlexer<'t>  {
    Self::new(StrTokenizer::from_str(s))
  }
  pub fn from_source(s:&'t LexSource<'t>) -> untypedlexer<'t>  {
    Self::new(StrTokenizer::from_source(s))
  }
  pub fn new(mut stk:StrTokenizer<'t>) -> untypedlexer<'t> {
    let mut keywords = HashSet::with_capacity(16);
    for kw in ["lambda","lam","Lam","let","in","define","lazy","weak","CBV",] {keywords.insert(kw);}
    for c in ['(',')','[',']','=',';','.',] {stk.add_single(c);}
    for d in [] {stk.add_double(d);}
    untypedlexer {stk,keywords}
  }
}
impl<'t> Tokenizer<'t,Term> for untypedlexer<'t>
{
   fn nextsym(&mut self) -> Option<TerminalToken<'t,Term>> {
    let tokopt = self.stk.next_token();
    if let None = tokopt {return None;}
    let token = tokopt.unwrap();
    match token.0 {
      RawToken::Alphanum(sym) if self.keywords.contains(sym) => Some(TerminalToken::from_raw(token,sym,<Term>::default())),
      RawToken::Num(n) => Some(TerminalToken::from_raw(token,"INTEGER",Const(n))),
      RawToken::Alphanum("liang") => Some(TerminalToken::from_raw(token,"Liang",Nothing)),
      RawToken::Alphanum(a) => Some(TerminalToken::from_raw(token,"ID",Var(str8::from(a)))),
      RawToken::Symbol(r".") => Some(TerminalToken::from_raw(token,"DOT",<Term>::default())),
      RawToken::Symbol(s) => Some(TerminalToken::from_raw(token,s,<Term>::default())),
      RawToken::Alphanum(s) => Some(TerminalToken::from_raw(token,s,<Term>::default())),
      _ => Some(TerminalToken::from_raw(token,"<LexicalError>",<Term>::default())),
    }
  }
   fn linenum(&self) -> usize {self.stk.line()}
   fn column(&self) -> usize {self.stk.column()}
   fn position(&self) -> usize {self.stk.current_position()}
}//impl Tokenizer

fn load_extras(parser:&mut ZCParser<Term,Vec<LBox<Term>>>)
{
}//end of load_extras: don't change this line as it affects augmentation
