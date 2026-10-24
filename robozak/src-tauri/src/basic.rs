use serde::Serialize;
use std::collections::{HashMap, HashSet};

const CMD_TABLE: &[u8] = include_bytes!("../resources/mrc3000.cmd");

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ParseError {
    pub line: usize,
    pub msg: String,
}

fn err<T>(line: usize, msg: &str) -> Result<T, ParseError> {
    Err(ParseError { line, msg: msg.to_string() })
}

/// Korean keyword -> English keyword, from the controller command table.
fn aliases() -> HashMap<String, String> {
    let (text, _, _) = encoding_rs::EUC_KR.decode(CMD_TABLE);
    text.lines()
        .filter_map(|l| l.split_once(','))
        .map(|(en, ko)| (ko.trim().to_uppercase(), en.trim().to_uppercase()))
        .filter(|(ko, en)| !ko.is_empty() && ko != en)
        .collect()
}

pub fn is_group(name: &str) -> bool {
    let b = name.as_bytes();
    b.len() >= 3
        && b[0] == b'G'
        && name[1..].trim_end_matches(|c: char| c.is_ascii_uppercase()).chars().all(|c| c.is_ascii_digit())
        && name[1..].chars().next().is_some_and(|c| c.is_ascii_digit())
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Id(String),
    Num(i32),
    Str(String),
    Op(String),
}

fn lex(line: &str, ln: usize, al: &HashMap<String, String>) -> Result<Vec<Tok>, ParseError> {
    let cs: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '\'' || c == '\u{2018}' || c == '\u{2019}' {
            break;
        } else if c == '"' {
            let start = i + 1;
            i = start;
            while i < cs.len() && cs[i] != '"' {
                i += 1;
            }
            if i >= cs.len() {
                return err(ln, "No end of strings.");
            }
            out.push(Tok::Str(cs[start..i].iter().collect()));
            i += 1;
        } else if c.is_ascii_digit() || (c == '&' && matches!(cs.get(i + 1), Some('H' | 'h'))) {
            let (radix, start) = if c == '&' { (16, i + 2) } else { (10, i) };
            i = start;
            while i < cs.len() && cs[i].is_digit(radix) {
                i += 1;
            }
            let s: String = cs[start..i].iter().collect();
            match i32::from_str_radix(&s, radix) {
                Ok(n) => out.push(Tok::Num(n)),
                Err(_) => return err(ln, "Wrong number."),
            }
        } else if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let w: String = cs[start..i].iter().collect::<String>().to_uppercase();
            if w == "REMARK" || w == "REM" || al.get(&w).is_some_and(|e| e == "REMARK") {
                break;
            }
            out.push(Tok::Id(al.get(&w).cloned().unwrap_or(w)));
        } else {
            let two: String = cs[i..(i + 2).min(cs.len())].iter().collect();
            if matches!(two.as_str(), "<>" | "<=" | ">=" | "<<" | ">>") {
                out.push(Tok::Op(two));
                i += 2;
            } else if "+-*/=<>(),;:&|".contains(c) {
                out.push(Tok::Op(c.to_string()));
                i += 1;
            } else {
                return err(ln, "Wrong symbol.");
            }
        }
    }
    Ok(out)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expr {
    Num(i32),
    Str(String),
    Var(String),
    Un(String, Box<Expr>),
    Bin(String, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Op {
    Assign(String, Expr),
    Cmd(String, Vec<Option<Expr>>),
    Goto(String),
    Gosub(String),
    Return,
    On { expr: Expr, labels: Vec<String>, gosub: bool },
    Jmp(usize),
    JmpFalse(Expr, usize),
    For { var: String, from: Expr, to: Expr, step: Option<Expr>, exit: usize },
    Next { var: String, start: usize },
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Ty {
    Byte,
    Int,
}

#[derive(Debug, Default, Serialize)]
pub struct Program {
    pub ops: Vec<Op>,
    pub lines: Vec<usize>,
    pub labels: HashMap<String, usize>,
    pub vars: HashMap<String, Ty>,
    pub consts: HashMap<String, i32>,
}

struct Parser {
    keywords: HashSet<String>,
    prog: Program,
    ln: usize,
    ifs: Vec<IfFrame>,
    fors: Vec<(String, usize)>,
}

struct IfFrame {
    pending: Option<usize>,
    ends: Vec<usize>,
}

struct Cur<'a> {
    t: &'a [Tok],
    p: usize,
}

impl<'a> Cur<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.t.get(self.p)
    }
    fn is_op(&self, o: &str) -> bool {
        matches!(self.peek(), Some(Tok::Op(x)) if x == o)
    }
    fn is_id(&self, k: &str) -> bool {
        matches!(self.peek(), Some(Tok::Id(x)) if x == k)
    }
    fn eat_op(&mut self, o: &str) -> bool {
        let r = self.is_op(o);
        if r {
            self.p += 1;
        }
        r
    }
    fn eat_id(&mut self, k: &str) -> bool {
        let r = self.is_id(k);
        if r {
            self.p += 1;
        }
        r
    }
    fn done(&self) -> bool {
        self.p >= self.t.len()
    }
}

impl Parser {
    fn emit(&mut self, op: Op) -> usize {
        self.prog.ops.push(op);
        self.prog.lines.push(self.ln);
        self.prog.ops.len() - 1
    }

    fn here(&self) -> usize {
        self.prog.ops.len()
    }

    fn patch(&mut self, at: usize, target: usize) {
        match &mut self.prog.ops[at] {
            Op::Jmp(t) | Op::JmpFalse(_, t) => *t = target,
            Op::For { exit, .. } => *exit = target,
            _ => {}
        }
    }

    fn declared(&self, n: &str) -> bool {
        self.prog.vars.contains_key(n) || self.prog.consts.contains_key(n)
    }

    fn ident(&self, c: &mut Cur) -> Result<String, ParseError> {
        match c.peek() {
            Some(Tok::Id(n)) => {
                let n = n.clone();
                c.p += 1;
                Ok(n)
            }
            _ => err(self.ln, "Wrong variable name."),
        }
    }

    fn expr(&self, c: &mut Cur) -> Result<Expr, ParseError> {
        self.bin(c, 0)
    }

    fn bin(&self, c: &mut Cur, level: usize) -> Result<Expr, ParseError> {
        const LEVELS: &[&[&str]] = &[
            &["OR", "XOR", "|"],
            &["AND", "&"],
            &["=", "<>", "<", ">", "<=", ">="],
            &["<<", ">>"],
            &["+", "-"],
            &["*", "/", "MOD"],
        ];
        if level == LEVELS.len() {
            return self.unary(c);
        }
        let mut lhs = self.bin(c, level + 1)?;
        loop {
            let op = match c.peek() {
                Some(Tok::Op(o)) | Some(Tok::Id(o)) if LEVELS[level].contains(&o.as_str()) => o.clone(),
                _ => return Ok(lhs),
            };
            c.p += 1;
            let rhs = self.bin(c, level + 1)?;
            lhs = Expr::Bin(op, Box::new(lhs), Box::new(rhs));
        }
    }

    fn unary(&self, c: &mut Cur) -> Result<Expr, ParseError> {
        if c.eat_id("NOT") {
            return Ok(Expr::Un("NOT".into(), Box::new(self.unary(c)?)));
        }
        if c.eat_op("-") {
            return Ok(Expr::Un("-".into(), Box::new(self.unary(c)?)));
        }
        self.primary(c)
    }

    fn primary(&self, c: &mut Cur) -> Result<Expr, ParseError> {
        match c.peek().cloned() {
            Some(Tok::Num(n)) => {
                c.p += 1;
                Ok(Expr::Num(n))
            }
            Some(Tok::Str(s)) => {
                c.p += 1;
                Ok(Expr::Str(s))
            }
            Some(Tok::Op(o)) if o == "(" => {
                c.p += 1;
                let e = self.expr(c)?;
                if !c.eat_op(")") {
                    return err(self.ln, "Wrong expression.");
                }
                Ok(e)
            }
            Some(Tok::Id(n)) => {
                c.p += 1;
                if c.eat_op("(") {
                    let mut args = Vec::new();
                    if !c.eat_op(")") {
                        loop {
                            args.push(self.expr(c)?);
                            if c.eat_op(")") {
                                break;
                            }
                            if !c.eat_op(",") {
                                return err(self.ln, "Wrong syntax.");
                            }
                        }
                    }
                    Ok(Expr::Call(n, args))
                } else if self.declared(&n) || is_group(&n) || self.keywords.contains(&n) {
                    Ok(Expr::Var(n))
                } else {
                    err(self.ln, "This variable name(or constant name) is not declared.")
                }
            }
            _ => err(self.ln, "Wrong expression."),
        }
    }

    fn args(&self, c: &mut Cur) -> Result<Vec<Option<Expr>>, ParseError> {
        let mut out = Vec::new();
        if c.done() {
            return Ok(out);
        }
        loop {
            if c.is_op(",") || c.done() {
                out.push(None);
            } else {
                out.push(Some(self.expr(c)?));
            }
            if c.done() {
                return Ok(out);
            }
            if !c.eat_op(",") && !c.eat_op(";") {
                return err(self.ln, "Wrong command syntax.");
            }
            if c.done() {
                out.push(None);
                return Ok(out);
            }
        }
    }

    fn label_list(&self, c: &mut Cur) -> Result<Vec<String>, ParseError> {
        let mut v = vec![self.ident(c)?];
        while c.eat_op(",") {
            v.push(self.ident(c)?);
        }
        Ok(v)
    }

    fn line(&mut self, toks: &[Tok]) -> Result<(), ParseError> {
        let mut c = Cur { t: toks, p: 0 };
        let Some(Tok::Id(first)) = c.peek().cloned() else {
            return err(self.ln, "Wrong command syntax.");
        };
        if toks.len() == 2 && toks[1] == Tok::Op(":".into()) && !self.keywords.contains(&first) {
            if self.prog.labels.contains_key(&first) {
                return err(self.ln, "Exist same label name already.");
            }
            let here = self.here();
            self.prog.labels.insert(first, here);
            return Ok(());
        }
        c.p += 1;
        match first.as_str() {
            "DIM" => loop {
                let n = self.ident(&mut c)?;
                if !c.eat_id("AS") {
                    return err(self.ln, "Wrong declaration of variable or constant.");
                }
                let ty = match self.ident(&mut c)?.as_str() {
                    "BYTE" => Ty::Byte,
                    "INTEGER" => Ty::Int,
                    _ => return err(self.ln, "Wrong declaration of variable or constant."),
                };
                if self.declared(&n) {
                    return err(self.ln, "This variable name(or constant name) is declared already.");
                }
                self.prog.vars.insert(n, ty);
                if !c.eat_op(",") {
                    break;
                }
            },
            "CONST" => {
                let n = self.ident(&mut c)?;
                if !c.eat_op("=") {
                    return err(self.ln, "Wrong declaration of variable or constant.");
                }
                let e = self.expr(&mut c)?;
                let v = const_eval(&e, &self.prog.consts).ok_or(ParseError {
                    line: self.ln,
                    msg: "Only number or constant.".into(),
                })?;
                if self.declared(&n) {
                    return err(self.ln, "This variable name(or constant name) is declared already.");
                }
                self.prog.consts.insert(n, v);
            }
            "IF" => return self.if_stmt(&mut c),
            "ELSEIF" => {
                let cond = self.expr(&mut c)?;
                if !c.eat_id("THEN") {
                    return err(self.ln, "THEN is not exist.");
                }
                let j = self.emit(Op::Jmp(0));
                let Some(f) = self.ifs.last_mut() else {
                    return err(self.ln, "No exist IF.");
                };
                f.ends.push(j);
                let pending = f.pending.take();
                if let Some(p) = pending {
                    let h = self.here();
                    self.patch(p, h);
                }
                let jf = self.emit(Op::JmpFalse(cond, 0));
                self.ifs.last_mut().unwrap().pending = Some(jf);
            }
            "ELSE" => {
                let j = self.emit(Op::Jmp(0));
                let Some(f) = self.ifs.last_mut() else {
                    return err(self.ln, "No exist IF.");
                };
                f.ends.push(j);
                let pending = f.pending.take();
                if let Some(p) = pending {
                    let h = self.here();
                    self.patch(p, h);
                }
            }
            "ENDIF" => {
                let Some(f) = self.ifs.pop() else {
                    return err(self.ln, "No exist IF.");
                };
                let h = self.here();
                for p in f.pending.into_iter().chain(f.ends) {
                    self.patch(p, h);
                }
            }
            "FOR" => {
                let v = self.ident(&mut c)?;
                if !self.prog.vars.contains_key(&v) {
                    return err(self.ln, "This variable name(or constant name) is not declared.");
                }
                if !c.eat_op("=") {
                    return err(self.ln, "Use 'variable = ?'");
                }
                let from = self.expr(&mut c)?;
                if !c.eat_id("TO") {
                    return err(self.ln, "Wrong syntax.");
                }
                let to = self.expr(&mut c)?;
                let step = if c.eat_id("STEP") { Some(self.expr(&mut c)?) } else { None };
                let at = self.emit(Op::For { var: v.clone(), from, to, step, exit: 0 });
                self.fors.push((v, at));
            }
            "NEXT" => {
                let v = self.ident(&mut c)?;
                let Some((fv, at)) = self.fors.pop() else {
                    return err(self.ln, "FOR is not exist.");
                };
                if fv != v {
                    return err(self.ln, "Check variable name in FOR and NEXT");
                }
                self.emit(Op::Next { var: v, start: at });
                let h = self.here();
                self.patch(at, h);
            }
            "GOTO" => {
                let l = self.ident(&mut c)?;
                self.emit(Op::Goto(l));
            }
            "GOSUB" => {
                let l = self.ident(&mut c)?;
                self.emit(Op::Gosub(l));
            }
            "RETURN" => {
                self.emit(Op::Return);
            }
            "END" => {
                self.emit(Op::End);
            }
            "ON" if toks.iter().any(|t| matches!(t, Tok::Id(k) if k == "GOTO" || k == "GOSUB")) => {
                let expr = self.expr(&mut c)?;
                let gosub = c.is_id("GOSUB");
                if !c.eat_id("GOTO") && !c.eat_id("GOSUB") {
                    return err(self.ln, "Wrong syntax.");
                }
                let labels = self.label_list(&mut c)?;
                self.emit(Op::On { expr, labels, gosub });
            }
            _ if c.is_op("=") => {
                if self.prog.consts.contains_key(&first) {
                    return err(self.ln, "Can't assign value at constant.");
                }
                if !self.prog.vars.contains_key(&first) {
                    return err(self.ln, "This variable name(or constant name) is not declared.");
                }
                c.p += 1;
                let e = self.expr(&mut c)?;
                self.emit(Op::Assign(first, e));
            }
            _ if self.keywords.contains(&first) => {
                let args = self.args(&mut c)?;
                self.emit(Op::Cmd(first, args));
                return Ok(());
            }
            _ => return err(self.ln, "Unknown contents."),
        }
        if !c.done() {
            return err(self.ln, "Wrong command syntax.");
        }
        Ok(())
    }

    fn if_stmt(&mut self, c: &mut Cur) -> Result<(), ParseError> {
        let cond_end = c.t.iter().position(|t| matches!(t, Tok::Id(k) if k == "THEN"));
        let Some(then_at) = cond_end else {
            return err(self.ln, "THEN is not exist.");
        };
        let cond = self.expr(&mut Cur { t: &c.t[..then_at], p: c.p })?;
        let rest = &c.t[then_at + 1..];
        let jf = self.emit(Op::JmpFalse(cond, 0));
        if rest.is_empty() {
            self.ifs.push(IfFrame { pending: Some(jf), ends: vec![] });
            return Ok(());
        }
        let else_at = rest.iter().position(|t| matches!(t, Tok::Id(k) if k == "ELSE"));
        let (then_part, else_part) = match else_at {
            Some(i) => (&rest[..i], Some(&rest[i + 1..])),
            None => (rest, None),
        };
        self.line(then_part)?;
        if let Some(e) = else_part {
            let j = self.emit(Op::Jmp(0));
            let h = self.here();
            self.patch(jf, h);
            self.line(e)?;
            let h = self.here();
            self.patch(j, h);
        } else {
            let h = self.here();
            self.patch(jf, h);
        }
        Ok(())
    }
}

fn const_eval(e: &Expr, consts: &HashMap<String, i32>) -> Option<i32> {
    match e {
        Expr::Num(n) => Some(*n),
        Expr::Var(v) => consts.get(v).copied(),
        Expr::Un(o, a) => {
            let a = const_eval(a, consts)?;
            Some(if o == "-" { a.wrapping_neg() } else { !a })
        }
        Expr::Bin(o, a, b) => apply(o, const_eval(a, consts)?, const_eval(b, consts)?),
        _ => None,
    }
}

fn apply(op: &str, a: i32, b: i32) -> Option<i32> {
    Some(match op {
        "+" => a.wrapping_add(b),
        "-" => a.wrapping_sub(b),
        "*" => a.wrapping_mul(b),
        "/" => a.checked_div(b).unwrap_or(0),
        "MOD" => a.checked_rem(b).unwrap_or(0),
        "AND" | "&" => a & b,
        "OR" | "|" => a | b,
        "XOR" => a ^ b,
        "<<" => a.wrapping_shl(b as u32),
        ">>" => a.wrapping_shr(b as u32),
        "=" => (a == b) as i32,
        "<>" => (a != b) as i32,
        "<" => (a < b) as i32,
        ">" => (a > b) as i32,
        "<=" => (a <= b) as i32,
        ">=" => (a >= b) as i32,
        _ => return None,
    })
}

pub fn parse(src: &str, keywords: &[String]) -> Result<Program, ParseError> {
    let al = aliases();
    let mut p = Parser {
        keywords: keywords.iter().cloned().collect(),
        prog: Program::default(),
        ln: 0,
        ifs: Vec::new(),
        fors: Vec::new(),
    };
    for (i, raw) in src.lines().enumerate() {
        p.ln = i + 1;
        let toks = lex(raw, p.ln, &al)?;
        if toks.is_empty() {
            continue;
        }
        p.line(&toks)?;
    }
    if !p.ifs.is_empty() {
        return err(p.ln, "No exist ENDIF.");
    }
    if !p.fors.is_empty() {
        return err(p.ln, "NEXT is not exist.");
    }
    for (i, op) in p.prog.ops.iter().enumerate() {
        let used: Vec<&String> = match op {
            Op::Goto(l) | Op::Gosub(l) => vec![l],
            Op::On { labels, .. } => labels.iter().collect(),
            _ => vec![],
        };
        if used.iter().any(|l| !p.prog.labels.contains_key(*l)) {
            return err(p.prog.lines[i], "No exist used label name.");
        }
    }
    Ok(p.prog)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Arg {
    Int(i32),
    Str(String),
    Group(String),
    Word(String),
    Empty,
}

pub trait Host {
    fn command(&mut self, name: &str, args: &[Arg]) -> Result<(), String>;
    fn function(&mut self, name: &str, args: &[Arg]) -> Result<i32, String>;
}

#[derive(Debug, PartialEq, Serialize)]
pub struct RunError {
    pub line: usize,
    pub msg: String,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Stop {
    End,
    StepLimit,
}

pub struct Interp<'a, H: Host> {
    prog: &'a Program,
    host: &'a mut H,
    pub vars: HashMap<String, i32>,
    pc: usize,
    calls: Vec<usize>,
    loops: HashMap<usize, i32>,
}

impl<'a, H: Host> Interp<'a, H> {
    pub fn new(prog: &'a Program, host: &'a mut H) -> Self {
        let vars = prog.vars.keys().map(|k| (k.clone(), 0)).collect();
        Interp { prog, host, vars, pc: 0, calls: Vec::new(), loops: HashMap::new() }
    }

    fn eval(&mut self, e: &Expr) -> Result<i32, String> {
        match self.arg(e)? {
            Arg::Int(n) => Ok(n),
            _ => Err("Only variable, constant and number.".into()),
        }
    }

    fn arg(&mut self, e: &Expr) -> Result<Arg, String> {
        Ok(match e {
            Expr::Num(n) => Arg::Int(*n),
            Expr::Str(s) => Arg::Str(s.clone()),
            Expr::Var(v) => {
                if let Some(n) = self.vars.get(v).or(self.prog.consts.get(v)) {
                    Arg::Int(*n)
                } else if is_group(v) {
                    Arg::Group(v.clone())
                } else {
                    Arg::Word(v.clone())
                }
            }
            Expr::Un(o, a) => {
                let a = self.eval(a)?;
                Arg::Int(if o == "-" { a.wrapping_neg() } else { (a == 0) as i32 })
            }
            Expr::Bin(o, a, b) => {
                let (a, b) = (self.eval(a)?, self.eval(b)?);
                Arg::Int(apply(o, a, b).ok_or("Wrong expression.")?)
            }
            Expr::Call(n, args) => {
                let args = args.iter().map(|a| self.arg(a)).collect::<Result<Vec<_>, _>>()?;
                Arg::Int(self.host.function(n, &args)?)
            }
        })
    }

    fn jump(&self, label: &str) -> usize {
        self.prog.labels[label]
    }

    pub fn run(&mut self, max_steps: usize) -> Result<Stop, RunError> {
        for _ in 0..max_steps {
            let Some(op) = self.prog.ops.get(self.pc) else {
                return Ok(Stop::End);
            };
            let line = self.prog.lines[self.pc];
            let at = |msg: String| RunError { line, msg };
            let mut next = self.pc + 1;
            match op {
                Op::Assign(v, e) => {
                    let n = self.eval(e).map_err(at)?;
                    let n = match self.prog.vars[v] {
                        Ty::Byte => n & 0xFF,
                        Ty::Int => n as i16 as i32,
                    };
                    self.vars.insert(v.clone(), n);
                }
                Op::Cmd(name, args) => {
                    let mut a = Vec::new();
                    for x in args {
                        a.push(match x {
                            Some(x) => self.arg(x).map_err(at)?,
                            None => Arg::Empty,
                        });
                    }
                    self.host.command(name, &a).map_err(at)?;
                }
                Op::Goto(l) => next = self.jump(l),
                Op::Gosub(l) => {
                    self.calls.push(next);
                    next = self.jump(l);
                }
                Op::Return => match self.calls.pop() {
                    Some(r) => next = r,
                    None => return Err(at("RETURN without GOSUB.".into())),
                },
                Op::On { expr, labels, gosub } => {
                    let n = self.eval(expr).map_err(at)?;
                    if n >= 1 && (n as usize) <= labels.len() {
                        if *gosub {
                            self.calls.push(next);
                        }
                        next = self.jump(&labels[n as usize - 1]);
                    }
                }
                Op::Jmp(t) => next = *t,
                Op::JmpFalse(c, t) => {
                    if self.eval(c).map_err(at)? == 0 {
                        next = *t;
                    }
                }
                Op::For { var, from, to, step, exit } => {
                    let f = self.eval(from).map_err(at)?;
                    let t = self.eval(to).map_err(at)?;
                    let s = match step {
                        Some(s) => self.eval(s).map_err(at)?,
                        None => 1,
                    };
                    self.vars.insert(var.clone(), f);
                    self.loops.insert(self.pc, t);
                    if (s >= 0 && f > t) || (s < 0 && f < t) {
                        next = *exit;
                    }
                }
                Op::Next { var, start } => {
                    let Op::For { step, .. } = &self.prog.ops[*start] else { unreachable!() };
                    let s = match step {
                        Some(s) => self.eval(s).map_err(at)?,
                        None => 1,
                    };
                    let t = self.loops[start];
                    let v = self.vars[var] + s;
                    self.vars.insert(var.clone(), v);
                    if (s >= 0 && v <= t) || (s < 0 && v >= t) {
                        next = start + 1;
                    }
                }
                Op::End => return Ok(Stop::End),
            }
            self.pc = next;
        }
        Ok(Stop::StepLimit)
    }
}

pub fn group_motors(name: &str) -> Vec<usize> {
    let digits: String = name[1..].chars().take_while(|c| c.is_ascii_digit()).collect();
    let n: usize = digits.parse().unwrap_or(0);
    let letter = name[1 + digits.len()..].chars().next();
    match (n, letter) {
        (6, Some(l)) | (8, Some(l)) => {
            let i = (l as u8 - b'A') as usize;
            (i * n..(i + 1) * n).collect()
        }
        (n, None) => (0..n).collect(),
        _ => vec![],
    }
}

#[derive(Debug, Serialize)]
pub struct SimRobot {
    pub servos: Vec<u8>,
    pub enabled: Vec<bool>,
    pub speed: i32,
    pub elapsed_ms: i64,
    pub output: Vec<String>,
    pub log: Vec<String>,
}

impl Default for SimRobot {
    fn default() -> Self {
        SimRobot {
            servos: vec![100; 32],
            enabled: vec![false; 32],
            speed: 5,
            elapsed_ms: 0,
            output: Vec::new(),
            log: Vec::new(),
        }
    }
}

fn int(a: &Arg) -> Option<i32> {
    if let Arg::Int(n) = a { Some(*n) } else { None }
}

impl Host for SimRobot {
    fn command(&mut self, name: &str, args: &[Arg]) -> Result<(), String> {
        match name {
            "MOVE" | "ZERO" | "POS" => {
                let (motors, vals) = match args.first() {
                    Some(Arg::Group(g)) => (group_motors(g), &args[1..]),
                    _ => ((0..6).collect(), args),
                };
                for (m, v) in motors.iter().zip(vals) {
                    if let Some(v) = int(v) {
                        self.servos[*m] = v.clamp(0, 254) as u8;
                    }
                }
            }
            "SERVO" => {
                if let [Arg::Int(m), Arg::Int(v)] = args {
                    *self.servos.get_mut(*m as usize).ok_or("Over range.")? = (*v).clamp(0, 254) as u8;
                }
            }
            "MOTOR" | "MOTOROFF" => {
                let on = name == "MOTOR";
                match args.first() {
                    Some(Arg::Group(g)) => group_motors(g).into_iter().for_each(|m| self.enabled[m] = on),
                    Some(Arg::Int(m)) => *self.enabled.get_mut(*m as usize).ok_or("Over range.")? = on,
                    _ => {}
                }
            }
            "SPEED" => self.speed = args.first().and_then(int).unwrap_or(self.speed),
            "DELAY" => self.elapsed_ms += args.first().and_then(int).unwrap_or(0) as i64,
            "PRINT" => self.output.push(
                args.iter()
                    .map(|a| match a {
                        Arg::Int(n) => n.to_string(),
                        Arg::Str(s) => s.clone(),
                        _ => String::new(),
                    })
                    .collect::<Vec<_>>()
                    .join(""),
            ),
            _ => self.log.push(format!("{name} {args:?}")),
        }
        Ok(())
    }

    fn function(&mut self, name: &str, _args: &[Arg]) -> Result<i32, String> {
        self.log.push(format!("{name}() -> 0"));
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kw() -> Vec<String> {
        crate::parse_keywords(include_str!("../resources/mrc3000e.cmd"))
    }

    fn run(src: &str) -> (HashMap<String, i32>, SimRobot) {
        let prog = parse(src, &kw()).unwrap();
        let mut sim = SimRobot::default();
        let vars = {
            let mut i = Interp::new(&prog, &mut sim);
            assert_eq!(i.run(100_000), Ok(Stop::End));
            i.vars
        };
        (vars, sim)
    }

    #[test]
    fn loops_if_and_subs() {
        let (v, _) = run("DIM I AS BYTE\nDIM S AS INTEGER\nFOR I = 1 TO 4\n IF I MOD 2 = 0 THEN S = S + I ELSE S = S - 1\nNEXT I\nGOSUB X\nEND\nX:\nS = S * 10\nRETURN");
        assert_eq!(v["S"], 40);
    }

    #[test]
    fn block_if_and_on_goto() {
        let (v, _) = run("DIM A AS BYTE\nDIM B AS BYTE\nA = 2\nON A GOTO L1,L2\nL1:\nB = 1\nEND\nL2:\nIF A = 1 THEN\nB = 5\nELSEIF A = 2 THEN\nB = 7\nELSE\nB = 9\nENDIF\nEND");
        assert_eq!(v["B"], 7);
    }

    #[test]
    fn byte_wraps_and_korean_keywords() {
        let (v, _) = run("선언 A 는 바이트\nA = 250 + 10\n끝");
        assert_eq!(v["A"], 4);
    }

    #[test]
    fn move_updates_servos() {
        let (_, sim) = run("DIM A AS BYTE\nMOTOR G6A\nMOVE G6A,10,20,30,40,50,60\nSERVO 23, 77\nDELAY 100\nPRINT \"ok\"");
        assert_eq!(&sim.servos[..6], &[10, 20, 30, 40, 50, 60]);
        assert_eq!(sim.servos[23], 77);
        assert!(sim.enabled[0] && !sim.enabled[6]);
        assert_eq!(sim.elapsed_ms, 100);
        assert_eq!(sim.output, vec!["ok"]);
    }

    #[test]
    fn syntax_errors_have_lines() {
        let e = parse("DIM A AS BYTE\nA = B\n", &kw()).unwrap_err();
        assert_eq!(e.line, 2);
        assert!(parse("GOTO NOPE", &kw()).is_err());
        assert!(parse("IF 1 = 1 THEN\nEND", &kw()).is_err());
        assert!(parse("FOR I = 1 TO 2", &kw()).is_err());
    }

    #[test]
    fn robot_headers_parse() {
        for src in [include_str!("../resources/rrm_header_pc.bas"), include_str!("../resources/rrm_header_ir.bas")] {
            let mut src = src.to_string();
            for n in 1..=32 {
                src.push_str(&format!("\nK{n}:\nGOTO MAIN_LOOP"));
            }
            parse(&src, &kw()).unwrap();
        }
    }
}
