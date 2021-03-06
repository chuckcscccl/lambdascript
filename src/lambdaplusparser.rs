//Parser generated by rustlr

#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
extern crate rustlr;
use rustlr::{RuntimeParser,RProduction,Stateaction,decode_action};
use std::rc::Rc;
use rustlr::LRc;
use crate::abstmachine::*;
use crate::abstmachine::Lamterm::*;
use crate::abstmachine::Tlist::*;

const SYMBOLS:[&'static str;19] = ["lambda","(",")","[","]","DOT","let","=","in","define",";","INTEGER","ID","T","Vars","Moreargs","Ts","START","EOF"];

const TABLE:[u64;157] = [4295360512,55834836993,47244967936,38655164416,25769934848,68719673345,51540131840,65536,281526516908032,281535106908161,563001493749760,844480765493249,844472175099904,844424930197504,844502239543299,844450700066816,844476470263808,844429225492480,844463585296384,1125942857367552,1407379178717186,1407413538455554,1407426423357442,1407409243488258,1407400653553666,1407422128390146,1407383473684482,1407417833422850,1407374883749890,1688901400395776,1688849860329472,1688854155624448,1688897105231872,1688888515428352,1688875630198784,1688905695756289,1970376377565184,2251804108783618,2251808403750914,2251834173554690,2251851353423874,2251825583620098,2251799813816322,2251847058456578,2251838468521986,2251842763489282,2533296265887746,2533326330658818,2814801307762688,2814771243057152,3096254809767936,3377742671446016,3659221941878786,3659174697238530,3659200467042306,3659252006649858,3659226236846082,3659178992205826,3659213351944194,3940649674473474,3940675444277250,3940696919113730,3940688329179138,3940701214081026,3940714099769345,3940653969440770,3940658264408066,4222154716807168,4503621102927874,4503651167698946,4785126144212992,4785078899441664,4785100374016000,4785121849049088,4785130440097793,4785074604146688,4785113259245568,5066601120923648,5066605416873985,5066553876152320,5066596825759744,5066588235956224,5066549580857344,5066575350726656,5348071802208258,5348024557568002,5348063212273666,5348028852535298,5348101866979330,5348076097175554,5348050327371778,5629525304147968,5629546779181056,5629555370360833,5629551074344960,5629503829573632,5629538189377536,5629499534278656,5629508125786112,5911030347202561,5910974510989312,5911000280858624,5911013166088192,5910978806284288,5911021755891712,5911026051055616,6192492437635074,6192483847700482,6192501027569666,6192458077896706,6192488142667778,6192449487962114,6192475257765890,6192453782929410,6192496732602370,6473958825852928,6755450981253122,6755403736612866,6755446686285826,6755399441645570,6755425211449346,6755438096351234,6755408031580162,7036913072734210,7036874418028546,7036900187832322,7036921662668802,7036883007963138,7036878712995842,7036917367701506,7036908777766914,7036925957636098,7318383754674178,7318353689903106,7318400934543362,7318396639576066,7318357984870402,7318375164739586,7318388049641474,7318349394935810,7318392344608770,7599880207597569,7599828666548224,7599863026352128,7599850141122560,7599824371253248,7599875911319552,7599871616155648,7881307938226178,7881338002997250,7881325118095362,7881346592931842,7881350887899138,7881303643258882,7881333708029954,7881299348291586,7881342297964546,];

pub fn make_parser() -> RuntimeParser<Lamterm,Machine>
{
 let mut parser1:RuntimeParser<Lamterm,Machine> = RuntimeParser::new(13,29);
 let mut rule = RProduction::<Lamterm,Machine>::new_skeleton("start");
 rule = RProduction::<Lamterm,Machine>::new_skeleton("Ts");
 rule.Ruleaction = |parser|{ parser.stack.pop();   let x:Lamterm=parser.stack.pop().unwrap().value;   Terms(cons(x,Tlnil)) };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("Ts");
 rule.Ruleaction = |parser|{ parser.stack.pop();   let x:Lamterm=parser.stack.pop().unwrap().value;   let vs:Lamterm=parser.stack.pop().unwrap().value;   Terms(cons(x,gettlist(vs))) };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("T");
 rule.Ruleaction = |parser|{  let x:Lamterm=parser.stack.pop().unwrap().value;   x };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("T");
 rule.Ruleaction = |parser|{  let x:Lamterm=parser.stack.pop().unwrap().value;   x };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("T");
 rule.Ruleaction = |parser|{ parser.stack.pop();   let ts:Lamterm=parser.stack.pop().unwrap().value;   let a:Lamterm=parser.stack.pop().unwrap().value;  parser.stack.pop();   formapp(a,ts) };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("T");
 rule.Ruleaction = |parser|{  let b:Lamterm=parser.stack.pop().unwrap().value;  parser.stack.pop();   let vs:Lamterm=parser.stack.pop().unwrap().value;  parser.stack.pop();   formabs(vs,b) };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("T");
 rule.Ruleaction = |parser|{  let b:Lamterm=parser.stack.pop().unwrap().value;  parser.stack.pop();   let v:Lamterm=parser.stack.pop().unwrap().value;  parser.stack.pop();   let x:Lamterm=parser.stack.pop().unwrap().value;  parser.stack.pop();  
  let labs = parser.lrc(Abs(getstr(&x),parser.lrc(b)));
  App(labs,parser.lrc(v)) };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("T");
 rule.Ruleaction = |parser|{  let v:Lamterm=parser.stack.pop().unwrap().value;  parser.stack.pop();   let x:Lamterm=parser.stack.pop().unwrap().value;  parser.stack.pop();  
  parser.exstate.Globals = Some(Rc::new(addbind(getstr(&x),Rc::new(newclosure(parser.lrc(v),parser.exstate.Globals.clone())),&parser.exstate.Globals)));
  return Nothing; };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("Moreargs");
 rule.Ruleaction = |parser|{ Terms(Tlnil)};
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("Moreargs");
 rule.Ruleaction = |parser|{  let b:Lamterm=parser.stack.pop().unwrap().value;   let ms:Lamterm=parser.stack.pop().unwrap().value;   Terms(cons(b,gettlist(ms))) };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("Vars");
 rule.Ruleaction = |parser|{  let x:Lamterm=parser.stack.pop().unwrap().value;   Terms(cons(x,Tlnil)) };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("Vars");
 rule.Ruleaction = |parser|{  let y:Lamterm=parser.stack.pop().unwrap().value;   let vs:Lamterm=parser.stack.pop().unwrap().value;   Terms(cons(y,gettlist(vs))) };
 parser1.Rules.push(rule);
 rule = RProduction::<Lamterm,Machine>::new_skeleton("START");
 rule.Ruleaction = |parser|{ parser.stack.pop();   return <Lamterm>::default();};
 parser1.Rules.push(rule);
 parser1.Errsym = "";

 for i in 0..157 {
   let symi = ((TABLE[i] & 0x0000ffff00000000) >> 32) as usize;
   let sti = ((TABLE[i] & 0xffff000000000000) >> 48) as usize;
   parser1.RSM[sti].insert(SYMBOLS[symi],decode_action(TABLE[i]));
 }

 for s in SYMBOLS { parser1.Symset.insert(s); }

 load_extras(&mut parser1);
 return parser1;
} //make_parser

fn load_extras(parser:&mut RuntimeParser<Lamterm,Machine>)
{
}//end of load_extras: don't change this line as it affects augmentation
