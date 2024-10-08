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
use std::cell::RefCell;
use std::rc::Rc;
extern crate rustlr;
use crate::untyped::Term::*;
use crate::untyped::*;
use fixedstr::str16;
use rustlr::{decode_action, Stateaction, TerminalToken, Tokenizer, ZCParser, ZCRProduction};
use rustlr::{unbox, LBox};
use rustlr::{LexSource, RawToken, StrTokenizer};
use std::collections::{HashMap, HashSet};

static SYMBOLS: [&'static str; 29] = [
    "_WILDCARD_TOKEN_",
    "lambda",
    "lam",
    "Lam",
    "(",
    ")",
    "[",
    "]",
    "DOT",
    "let",
    "=",
    "in",
    "define",
    "lazy",
    "weak",
    "CBV",
    "Liang",
    ";",
    "INTEGER",
    "ID",
    "T",
    "F",
    "Fs",
    "TOP",
    "Vars",
    "LAMSYM",
    "Ts",
    "START",
    "EOF",
];

static TABLE: [u64; 214] = [
    64424968192,
    85900132353,
    107374837761,
    17180917760,
    51540459520,
    38654902272,
    81604968448,
    4295294976,
    94489673729,
    77309542400,
    98784968705,
    90194837505,
    60129804288,
    12885819392,
    8590917632,
    111669215233,
    281582351548417,
    281513631612928,
    281492157628416,
    281560876843009,
    281487862530048,
    281552286253056,
    281556581679104,
    281565171548161,
    281569466384385,
    281573762072577,
    281595235794947,
    281539401678848,
    281535106514944,
    281526517170176,
    281483567628288,
    281479272005632,
    563022968193026,
    562997198389250,
    563031558127618,
    563027263160322,
    562967133618178,
    562971428585474,
    844506535690240,
    1125981511811072,
    1125917087760384,
    1125977216385024,
    1125990102401025,
    1407456489046018,
    1688897105297410,
    1688931465232384,
    1688927169806336,
    1688940055887873,
    1688867041181696,
    1688922875101186,
    1688871335493634,
    1970342017892352,
    1970406441943040,
    1970415032664065,
    1970402146516992,
    2251872828260354,
    2251881418194946,
    2251821288652802,
    2251877123227650,
    2251847058456578,
    2251816993685506,
    2533296265494530,
    2533352100069378,
    2533322035298306,
    2533347805102082,
    2533356395036674,
    2533291970527234,
    2814852847763457,
    2814831372992512,
    3096297759834112,
    3377772736020482,
    3659256303321088,
    3659230533451776,
    3940731279572994,
    4222206256218114,
    4503694117044225,
    4503608218288128,
    4503689822208001,
    4503676936912896,
    4503616808288256,
    4503612513189888,
    4503707002208257,
    4503681232338944,
    4503638282272768,
    4503664052338688,
    4503659757174784,
    4503685528485889,
    4503603922665472,
    4785147620360192,
    5066592532365312,
    5348097572536322,
    5348046032928770,
    5348071802732546,
    5629546779049986,
    5629572548853762,
    5629521009246210,
    5629516714278914,
    5629576843821058,
    5629581138788354,
    5911021756088322,
    5910995986284546,
    5911047525892098,
    6192531093979136,
    6192483849404416,
    6473958824804354,
    6474006069444610,
    6755412325957634,
    6755481045434370,
    6755403736023042,
    6755476750467074,
    6755463865565186,
    6755519700140034,
    6755408030990338,
    6755416620924930,
    6755450980663298,
    6755459570597890,
    6755438095761410,
    7036956024242176,
    7318392346312704,
    7599845848252416,
    7881338002669570,
    7881359477506050,
    7881419607048194,
    7881303642931202,
    7881316527833090,
    7881307937898498,
    7881312232865794,
    7881350887571458,
    7881376657375234,
    7881363772473346,
    7881380952342530,
    8162838749577216,
    8162791505526784,
    8162778619904000,
    8162851634151424,
    8162782915526656,
    8162864519446529,
    8162860226248705,
    8162868814282753,
    8162855929577472,
    8162881699446785,
    8162834454413312,
    8162812979511296,
    8162787210428416,
    8444330906484738,
    8444283661844482,
    8725805882998784,
    8725741458948096,
    8725732868947968,
    8725784407834624,
    8725818767704065,
    8725788702998528,
    8725831652868097,
    8725728573325312,
    8725737163849728,
    8725801587572736,
    8725762932932608,
    8725814472867841,
    8725810179735553,
    9007242206838784,
    9288738656419840,
    9288687117271040,
    9288760133287937,
    9288768721125377,
    9288764426289153,
    9288781606289409,
    9288678526746624,
    9288712886353920,
    9288682822369280,
    9288751540994048,
    9288755836420096,
    9288691412369408,
    9288734361255936,
    9570226518032386,
    9570230812999682,
    9570222223065090,
    9570196453261314,
    9570166388490242,
    9570170683457538,
    9851671432069120,
    10133120637075458,
    10133172176683010,
    10133146406879234,
    10414578433589248,
    10414655743262720,
    10414651447836672,
    10414587024113664,
    10414681513132033,
    10414582729211904,
    10414612793196544,
    10414668627968001,
    10414660040261633,
    10414634268098560,
    10414664333131777,
    10414591319212032,
    10414638563262464,
    10696122130366466,
    10977541272633344,
    10977605696684032,
    10977532682633216,
    10977588516683776,
    10977609993748481,
    10977614286553089,
    10977631466553345,
    10977536977534976,
    10977618581389313,
    10977562746617856,
    10977601401257984,
    10977528387010560,
    10977584221519872,
    11259072083853314,
    11540495520825346,
    11540521290629122,
    11540547060432898,
];

pub fn make_parser() -> ZCParser<Term, Vec<LBox<Term>>> {
    let mut parser1: ZCParser<Term, Vec<LBox<Term>>> = ZCParser::new(21, 42);
    let mut rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("start");
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("Ts");
    rule.Ruleaction = |parser| {
        let mut _item1_ = parser.popstack();
        let mut x = parser.popstack();
        parser.exstate.push(x.lbox());
        Nothing
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("Ts");
    rule.Ruleaction = |parser| {
        let mut _item2_ = parser.popstack();
        let mut x = parser.popstack();
        let mut _item0_ = parser.popstack();
        parser.exstate.push(x.lbox());
        Nothing
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("Fs");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        if let (a,) = (_item0_.value,) {
            a
        } else {
            parser.bad_pattern("(a,)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("Fs");
    rule.Ruleaction = |parser| {
        let mut b = parser.popstack();
        let mut a = parser.popstack();
        App(a.lbox(), b.lbox())
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("F");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        if let ((x),) = (_item0_.value,) {
            x
        }
        /* var */
        else {
            parser.bad_pattern("((x),)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("F");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        if let ((x),) = (_item0_.value,) {
            x
        }
        /* const*/
        else {
            parser.bad_pattern("((x),)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("T");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        if let (a,) = (_item0_.value,) {
            a
        } else {
            parser.bad_pattern("(a,)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("F");
    rule.Ruleaction = |parser| {
        let mut _item2_ = parser.popstack();
        let mut _item1_ = parser.popstack();
        let mut _item0_ = parser.popstack();
        if let (a,) = (_item1_.value,) {
            a
        } else {
            parser.bad_pattern("(a,)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("T");
    rule.Ruleaction = |parser| {
        let mut x = parser.popstack();
        let mut _item0_ = parser.popstack();
        CBV(x.lbox())
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("T");
    rule.Ruleaction = |parser| {
        let mut x = parser.popstack();
        let mut _item0_ = parser.popstack();
        Weak(x.lbox())
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("T");
    rule.Ruleaction = |parser| {
        let mut b = parser.popstack();
        let mut _item2_ = parser.popstack();
        let mut _item1_ = parser.popstack();
        let mut _item0_ = parser.popstack();
        if let (Seq(mut vs),) = (_item1_.value,) {
            let mut t = b.value;
            while vs.len() > 0 {
                t = Abs(getvar(&unbox!(vs.pop().unwrap())), parser.lbx(0, t));
            }
            return t;
        } else {
            parser.bad_pattern("(Seq(mut vs),)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("Vars");
    rule.Ruleaction = |parser| {
        let mut x = parser.popstack();
        Seq(vec![x.lbox()])
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("Vars");
    rule.Ruleaction = |parser| {
        let mut y = parser.popstack();
        let mut _item0_ = parser.popstack();
        if let (Seq(mut vs),) = (_item0_.value,) {
            vs.push(y.lbox());
            Seq(vs)
        } else {
            parser.bad_pattern("(Seq(mut vs),)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("T");
    rule.Ruleaction = |parser| {
        let mut b = parser.popstack();
        let mut _item4_ = parser.popstack();
        let mut v = parser.popstack();
        let mut _item2_ = parser.popstack();
        let mut _item1_ = parser.popstack();
        let mut _item0_ = parser.popstack();
        if let (Var(x),) = (_item1_.value,) {
            App(parser.lbx(0, Abs(x, b.lbox())), v.lbox())
        } else {
            parser.bad_pattern("(Var(x),)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("TOP");
    rule.Ruleaction = |parser| {
        let mut v = parser.popstack();
        let mut _item2_ = parser.popstack();
        let mut _item1_ = parser.popstack();
        let mut _item0_ = parser.popstack();
        if let (Var(x),) = (_item1_.value,) {
            let nv = Def(true, x, v.lbox());
            //parser.exstate.push(parser.lbx(0,nv));
            nv
        } else {
            parser.bad_pattern("(Var(x),)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("TOP");
    rule.Ruleaction = |parser| {
        let mut v = parser.popstack();
        let mut _item3_ = parser.popstack();
        let mut _item2_ = parser.popstack();
        let mut _item1_ = parser.popstack();
        let mut _item0_ = parser.popstack();
        if let (Var(x),) = (_item2_.value,) {
            let nv = Def(false, x, v.lbox());
            nv
        } else {
            parser.bad_pattern("(Var(x),)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("TOP");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        if let ((x),) = (_item0_.value,) {
            x
        } else {
            parser.bad_pattern("((x),)")
        }
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("LAMSYM");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        <Term>::default()
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("LAMSYM");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        <Term>::default()
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("LAMSYM");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        <Term>::default()
    };
    parser1.Rules.push(rule);
    rule = ZCRProduction::<Term, Vec<LBox<Term>>>::new_skeleton("START");
    rule.Ruleaction = |parser| {
        let mut _item0_ = parser.popstack();
        <Term>::default()
    };
    parser1.Rules.push(rule);
    parser1.Errsym = "";
    parser1.resynch.insert(";");

    for i in 0..214 {
        let symi = ((TABLE[i] & 0x0000ffff00000000) >> 32) as usize;
        let sti = ((TABLE[i] & 0xffff000000000000) >> 48) as usize;
        parser1.RSM[sti].insert(SYMBOLS[symi], decode_action(TABLE[i]));
    }

    for s in SYMBOLS {
        parser1.Symset.insert(s);
    }

    load_extras(&mut parser1);
    return parser1;
} //make_parser

pub fn parse_with<'t>(
    parser: &mut ZCParser<Term, Vec<LBox<Term>>>,
    lexer: &mut dyn Tokenizer<'t, Term>,
) -> Result<Term, Term> {
    let _xres_ = parser.parse(lexer);
    if !parser.error_occurred() {
        Ok(_xres_)
    } else {
        Err(_xres_)
    }
} //parse_with public function

pub fn parse_train_with<'t>(
    parser: &mut ZCParser<Term, Vec<LBox<Term>>>,
    lexer: &mut dyn Tokenizer<'t, Term>,
    parserpath: &str,
) -> Result<Term, Term> {
    let _xres_ = parser.parse_train(lexer, parserpath);
    if !parser.error_occurred() {
        Ok(_xres_)
    } else {
        Err(_xres_)
    }
} //parse_train_with public function

// Lexical Scanner using RawToken and StrTokenizer
pub struct untypedlexer<'t> {
    stk: StrTokenizer<'t>,
    keywords: HashSet<&'static str>,
    lexnames: HashMap<&'static str, &'static str>,
    shared_state: Rc<RefCell<Vec<LBox<Term>>>>,
}
impl<'t> untypedlexer<'t> {
    pub fn from_str(s: &'t str) -> untypedlexer<'t> {
        Self::new(StrTokenizer::from_str(s))
    }
    pub fn from_source(s: &'t LexSource<'t>) -> untypedlexer<'t> {
        Self::new(StrTokenizer::from_source(s))
    }
    pub fn new(mut stk: StrTokenizer<'t>) -> untypedlexer<'t> {
        let mut lexnames = HashMap::with_capacity(64);
        let mut keywords = HashSet::with_capacity(64);
        let shared_state = Rc::new(RefCell::new(<Vec<LBox<Term>>>::default()));
        for kw in [
            "Lam",
            "_WILDCARD_TOKEN_",
            "lam",
            "define",
            "lambda",
            "CBV",
            "in",
            "let",
            "lazy",
            "weak",
        ] {
            keywords.insert(kw);
        }
        for c in ['(', ')', '[', ']', '=', ';', '.'] {
            stk.add_single(c);
        }
        for d in [] {
            stk.add_double(d);
        }
        for d in [] {
            stk.add_triple(d);
        }
        for (k, v) in [(r".", "DOT")] {
            lexnames.insert(k, v);
        }
        untypedlexer {
            stk,
            keywords,
            lexnames,
            shared_state,
        }
    }
}
impl<'t> Tokenizer<'t, Term> for untypedlexer<'t> {
    fn nextsym(&mut self) -> Option<TerminalToken<'t, Term>> {
        let tokopt = self.stk.next_token();
        if let None = tokopt {
            return None;
        }
        let token = tokopt.unwrap();
        match token.0 {
            RawToken::Alphanum(sym) if self.keywords.contains(sym) => {
                let truesym = self.lexnames.get(sym).unwrap_or(&sym);
                Some(TerminalToken::from_raw(token, truesym, <Term>::default()))
            }
            RawToken::Num(n) => Some(TerminalToken::from_raw(token, "INTEGER", Const(n))),
            RawToken::Alphanum("liang") => Some(TerminalToken::from_raw(token, "Liang", Nothing)),
            RawToken::Alphanum(a) => {
                Some(TerminalToken::from_raw(token, "ID", Var(str16::from(a))))
            }
            RawToken::Symbol(s) if self.lexnames.contains_key(s) => {
                let tname = self.lexnames.get(s).unwrap();
                Some(TerminalToken::from_raw(token, tname, <Term>::default()))
            }
            RawToken::Symbol(s) => Some(TerminalToken::from_raw(token, s, <Term>::default())),
            RawToken::Alphanum(s) => Some(TerminalToken::from_raw(token, s, <Term>::default())),
            _ => {
                let _rrodb = token.0.to_staticstr();
                Some(TerminalToken::from_raw(token, _rrodb, <Term>::default()))
            }
        }
    }
    fn linenum(&self) -> usize {
        self.stk.line()
    }
    fn column(&self) -> usize {
        self.stk.column()
    }
    fn position(&self) -> usize {
        self.stk.current_position()
    }
    fn current_line(&self) -> &str {
        self.stk.current_line()
    }
    fn get_line(&self, i: usize) -> Option<&str> {
        self.stk.get_line(i)
    }
    fn get_slice(&self, s: usize, l: usize) -> &str {
        self.stk.get_slice(s, l)
    }
} //impl Tokenizer

fn load_extras(parser: &mut ZCParser<Term, Vec<LBox<Term>>>) {} //end of load_extras: don't change this line as it affects augmentation
