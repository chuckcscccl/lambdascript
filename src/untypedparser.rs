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
use fixedstr::str16;

static SYMBOLS:[&'static str;29] = ["_WILDCARD_TOKEN_","lambda","lam","Lam","(",")","[","]","DOT","let","=","in","define","lazy","weak","CBV","Liang",";","INTEGER","ID","T","F","Fs","TOP","Vars","LAMSYM","Ts","START","EOF"];

static TABLE:[u64;214] = [64424902656,77310001152,12885557248,94489804801,60130590720,51539935232,4295098368,107375034369,85900132353,38655688704,111669411841,98784313345,8590655488,81605296128,17180065792,90194771969,281547992268800,563031558914050,844519419936769,844489355034624,844429225230336,844515124903937,844510830657537,844502240133120,844442110197760,844463585820672,844433520787456,844532305166337,844437815689216,844485060722688,844506535428096,1125998692335617,1125951446777856,1125908497498112,1125981512138752,1125960037433344,1125964331745280,1125977216843776,1125912792399872,1125904201940992,1125990101614593,1125938562531328,1126020165926915,1125994396647425,1125917086908416,1125985806974977,1126007281876993,1407430719438848,1407456489308160,1688940056018945,1688927170265088,1688867040329728,1688931465560064,1970372081745922,1970406441484290,1970402146516994,1970342016974850,1970397851549698,1970346311942146,2251881418981376,2251816993751040,2251877123686400,2251890009505793,2251821288914946,2251847058718722,2251872828522498,2533322035363842,2533356395102210,2533291970592770,2533352100134914,2533347805167618,2533296265560066,2814831372730370,3096306349375490,3377772736020482,3659256303255552,3659277778026497,3940696918851586,3940722688655362,3940731278589954,3940666854080514,3940726983622658,3940671149047810,4222206256742400,4503616807436288,4503689823453185,4503676937371648,4503681232666624,4785091783950338,4785126143688706,4785194863165442,4785139028590594,4785134733623298,4785083194015746,4785087488983042,4785078899048450,4785113258786818,4785156208459778,4785151913492482,5066571057463296,5348097573847040,5629581140557824,5911017462628352,6192470962995202,6192496732798978,6192522502602754,6473971709181954,6473941644410882,6473945939378178,6473997478985730,6474006068920322,6474001773953026,6755481047531520,6755433802956800,7036908778225666,7036956022865922,7318392346378240,7599897386221570,7599871616417794,7599845846614018,7881346592997378,7881372362801154,7881320823193602,7881316528226306,7881376657768450,7881380952735746,8162838749184002,8162851634085890,8162825864282114,8162787209576450,8162778619641858,8162894583758850,8162834454216706,8162812979380226,8162855929053186,8162782914609154,8162791504543746,8444292253286400,8725762933719040,8725784408621056,8725737163587584,8725728573128704,8725801588031488,8725818767835137,8725810179735553,8725831653064705,8725788702932992,8725805883326464,8725732868685824,8725741458096128,8725814472802305,9007280859906050,9007233615265794,9288764426223617,9288682822107136,9288691411517440,9288712887140352,9288781606486017,9288760133222401,9288734362042368,9288687117008896,9288738656354304,9288755836747776,9288678526550016,9288751541452800,9288768721256449,9570153503260672,9570209338753024,9570243697967105,9570157798817792,9570256583196673,9570230813458432,9570226518163456,9570213633064960,9570235109998593,9570239402934273,9570187863851008,9570162093719552,9570166388228096,9851710086774785,9851641364938752,9851628479971328,9851701494874112,9851662840561664,9851632775528448,9851637070430208,9851714379644929,9851731559907329,9851688609775616,9851684315463680,9851718674677761,9851705790169088,10133172176945154,10414595613786114,10414621383589890,10414647153393666,10696096362266624,10977597107142658,11259080673722368,11259106443460609,11259063493328896,11259016248492032,11259084970459137,11259076378427392,11259089263198209,11259007659081728,11259011953983488,11259003363524608,11259037724114944,11259059199016960,11259093558231041,11540495520825346,11540521290629122,11540547060432898,];

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

pub fn parse_with<'t>(parser:&mut ZCParser<Term,Vec<LBox<Term>>>, lexer:&mut untypedlexer<'t>) -> Result<Term,Term>
{
  let _xres_ = parser.parse(lexer);  if !parser.error_occurred() {Ok(_xres_)} else {Err(_xres_)}
}//parse_with public function

pub fn parse_train_with<'t>(parser:&mut ZCParser<Term,Vec<LBox<Term>>>, lexer:&mut untypedlexer<'t>, parserpath:&str) -> Result<Term,Term>
{
  let _xres_ = parser.parse_train(lexer,parserpath);  if !parser.error_occurred() {Ok(_xres_)} else {Err(_xres_)}
}//parse_train_with public function

// Lexical Scanner using RawToken and StrTokenizer
pub struct untypedlexer<'t> {
   stk: StrTokenizer<'t>,
   keywords: HashSet<&'static str>,
   lexnames: HashMap<&'static str,&'static str>,
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
    let mut lexnames = HashMap::with_capacity(64);
    let mut keywords = HashSet::with_capacity(64);
    for kw in ["_WILDCARD_TOKEN_","CBV","in","let","lam","lambda","define","weak","lazy","Lam",] {keywords.insert(kw);}
    for c in ['(',')','[',']','=',';','.',] {stk.add_single(c);}
    for d in [] {stk.add_double(d);}
    for d in [] {stk.add_triple(d);}
    for (k,v) in [(r".","DOT"),] {lexnames.insert(k,v);}
    untypedlexer {stk,keywords,lexnames}
  }
}
impl<'t> Tokenizer<'t,Term> for untypedlexer<'t>
{
   fn nextsym(&mut self) -> Option<TerminalToken<'t,Term>> {
    let tokopt = self.stk.next_token();
    if let None = tokopt {return None;}
    let token = tokopt.unwrap();
    match token.0 {
      RawToken::Alphanum(sym) if self.keywords.contains(sym) => {
        let truesym = self.lexnames.get(sym).unwrap_or(&sym);
        Some(TerminalToken::from_raw(token,truesym,<Term>::default()))
      },
      RawToken::Num(n) => Some(TerminalToken::from_raw(token,"INTEGER",Const(n))),
      RawToken::Alphanum("liang") => Some(TerminalToken::from_raw(token,"Liang",Nothing)),
      RawToken::Alphanum(a) => Some(TerminalToken::from_raw(token,"ID",Var(str16::from(a)))),
      RawToken::Symbol(s) if self.lexnames.contains_key(s) => {
        let tname = self.lexnames.get(s).unwrap();
        Some(TerminalToken::from_raw(token,tname,<Term>::default()))
      },
      RawToken::Symbol(s) => Some(TerminalToken::from_raw(token,s,<Term>::default())),
      RawToken::Alphanum(s) => Some(TerminalToken::from_raw(token,s,<Term>::default())),
      _ => Some(TerminalToken::from_raw(token,"<LexicalError>",<Term>::default())),
    }
  }
   fn linenum(&self) -> usize {self.stk.line()}
   fn column(&self) -> usize {self.stk.column()}
   fn position(&self) -> usize {self.stk.current_position()}
   fn current_line(&self) -> &str {self.stk.current_line()}
   fn get_line(&self,i:usize) -> Option<&str> {self.stk.get_line(i)}
}//impl Tokenizer

fn load_extras(parser:&mut ZCParser<Term,Vec<LBox<Term>>>)
{
}//end of load_extras: don't change this line as it affects augmentation
