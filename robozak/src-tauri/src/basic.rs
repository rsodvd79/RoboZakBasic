use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ParseError {
    pub line: usize,
    pub msg: String,
}

fn err<T>(line: usize, msg: &str) -> Result<T, ParseError> {
    Err(ParseError {
        line,
        msg: msg.to_string(),
    })
}

pub fn is_group(name: &str) -> bool {
    crate::compiler::group(name).is_some()
}

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Id(String),
    Num(i32),
    Str(String),
    Op(String),
}

fn tokens(toks: Vec<crate::compiler::Tok>, ln: usize) -> Result<Vec<Tok>, ParseError> {
    toks.into_iter()
        .map(|t| {
            Ok(match t {
                crate::compiler::Tok::Id(s) => Tok::Id(s),
                crate::compiler::Tok::Num(n) => {
                    Tok::Num(i32::try_from(n).map_err(|_| ParseError {
                        line: ln,
                        msg: "Costante fuori intervallo per la simulazione".into(),
                    })?)
                }
                crate::compiler::Tok::Str(s) => Tok::Str(s),
                crate::compiler::Tok::Op(s) => Tok::Op(s),
            })
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Expr {
    Num(i32),
    Str(String),
    Var(String),
    Bit(String, u8),
    Bin(String, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Op {
    Assign(String, Expr),
    AssignBit(String, u8, Expr),
    Cmd(String, Vec<Option<Expr>>),
    Goto(String),
    Gosub(String),
    Return,
    On {
        expr: Expr,
        labels: Vec<String>,
        gosub: bool,
    },
    Jmp(usize),
    JmpFalse(Expr, usize),
    For {
        var: String,
        from: Expr,
        to: Expr,
        step: Option<Expr>,
        exit: usize,
    },
    Next {
        var: String,
        start: usize,
    },
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
        let mut operators = Vec::new();
        let mut depth = 0;
        for (at, t) in c.t.iter().enumerate().skip(c.p) {
            match t {
                Tok::Op(o) if o == "(" => depth += 1,
                Tok::Op(o) if o == ")" && depth > 0 => depth -= 1,
                Tok::Op(o) if depth == 0 && [",", ";", ")"].contains(&o.as_str()) => break,
                Tok::Id(k)
                    if depth == 0 && ["THEN", "TO", "GOTO", "GOSUB"].contains(&k.as_str()) =>
                {
                    break;
                }
                Tok::Op(o)
                    if depth == 0 && ["=", "<>", "<", ">", "<=", ">="].contains(&o.as_str()) =>
                {
                    operators.push((at, true))
                }
                Tok::Id(o) if depth == 0 && ["AND", "OR", "XOR"].contains(&o.as_str()) => {
                    operators.push((at, false))
                }
                _ => {}
            }
        }
        let mut logical = Vec::new();
        let mut comparison = false;
        for (i, (at, relation)) in operators.iter().enumerate() {
            if *relation {
                comparison = true;
            } else if comparison && operators[i + 1..].iter().any(|(_, rel)| *rel) {
                logical.push(*at);
                comparison = false;
            }
        }
        self.binary(c, 1, &logical)
    }

    fn binary(&self, c: &mut Cur, minimum: u8, logical: &[usize]) -> Result<Expr, ParseError> {
        let mut lhs = self.primary(c)?;
        loop {
            let (op, priority) = match c.peek() {
                Some(Tok::Op(o)) | Some(Tok::Id(o))
                    if ["+", "-", "*", "/", "%", "MOD", "<<", ">>"].contains(&o.as_str()) =>
                {
                    (o.clone(), 3)
                }
                Some(Tok::Op(o)) if ["=", "<>", "<", ">", "<=", ">="].contains(&o.as_str()) => {
                    (o.clone(), 2)
                }
                Some(Tok::Id(o)) if ["AND", "OR", "XOR"].contains(&o.as_str()) => {
                    // Tra confronti sono logici; nelle formule numeriche sono operatori di bit.
                    (o.clone(), if logical.contains(&c.p) { 1 } else { 3 })
                }
                _ => return Ok(lhs),
            };
            if priority < minimum {
                return Ok(lhs);
            }
            c.p += 1;
            let rhs = self.binary(c, priority + 1, logical)?;
            lhs = Expr::Bin(op, Box::new(lhs), Box::new(rhs));
        }
    }

    fn bit(&self, c: &mut Cur, name: &str) -> Result<u8, ParseError> {
        let n = match c.peek() {
            Some(Tok::Num(n)) => Some(*n),
            Some(Tok::Id(n)) => self.prog.consts.get(n).copied(),
            _ => None,
        };
        let width = if self.prog.vars.get(name) == Some(&Ty::Int) {
            16
        } else {
            8
        };
        match n {
            Some(n) if (0..width).contains(&n) => {
                c.p += 1;
                Ok(n as u8)
            }
            _ => err(
                self.ln,
                "Indicatore di bit oltre la variabile: accesso non simulato",
            ),
        }
    }

    fn primary(&self, c: &mut Cur) -> Result<Expr, ParseError> {
        match c.peek().cloned() {
            Some(Tok::Num(n)) => {
                c.p += 1;
                Ok(Expr::Num(n))
            }
            Some(Tok::Str(s)) => {
                c.p += 1;
                s.bytes()
                    .next()
                    .map(|n| Expr::Num(n as i32))
                    .ok_or(ParseError {
                        line: self.ln,
                        msg: "Carattere ASCII mancante".into(),
                    })
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
                if c.eat_op(".") {
                    let bit = self.bit(c, &n)?;
                    return Ok(Expr::Bit(n, bit));
                }
                if let Some(value) = self.prog.consts.get(&n) {
                    return Ok(Expr::Num(*value));
                }
                if crate::compiler::func_code(&n).is_some_and(|(_, arg)| !arg)
                    && !self.prog.consts.contains_key(&n)
                {
                    return Ok(Expr::Call(n, Vec::new()));
                }
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
                    err(
                        self.ln,
                        "This variable name(or constant name) is not declared.",
                    )
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

    fn print_args(&self, c: &mut Cur) -> Result<Vec<Option<Expr>>, ParseError> {
        if c.is_id("FORMAT") {
            return Ok(vec![Some(self.primary(c)?)]);
        }
        let mut args = Vec::new();
        loop {
            args.push(Some(match c.peek() {
                Some(Tok::Str(s)) => {
                    let s = s.clone();
                    c.p += 1;
                    Expr::Str(s)
                }
                _ => self.primary(c)?,
            }));
            if !c.eat_op(",") {
                break;
            }
        }
        Ok(args)
    }

    fn line(&mut self, toks: &[Tok]) -> Result<(), ParseError> {
        let mut c = Cur { t: toks, p: 0 };
        let Some(Tok::Id(first)) = c.peek().cloned() else {
            return err(self.ln, "Wrong command syntax.");
        };
        if toks.len() == 2 && toks[1] == Tok::Op(":".into()) {
            if self.prog.labels.contains_key(&first) {
                return err(self.ln, "Exist same label name already.");
            }
            let here = self.here();
            self.prog.labels.insert(first, here);
            return Ok(());
        }
        c.p += 1;
        match first.as_str() {
            "REMARK" => return Ok(()),
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
                    return err(
                        self.ln,
                        "This variable name(or constant name) is not declared.",
                    );
                }
                if !c.eat_op("=") {
                    return err(self.ln, "Use 'variable = ?'");
                }
                let from = self.expr(&mut c)?;
                if !c.eat_id("TO") {
                    return err(self.ln, "Wrong syntax.");
                }
                let to = self.expr(&mut c)?;
                let step = if c.eat_id("STEP") {
                    Some(self.expr(&mut c)?)
                } else {
                    None
                };
                let at = self.emit(Op::For {
                    var: v.clone(),
                    from,
                    to,
                    step,
                    exit: 0,
                });
                self.fors.push((v, at));
            }
            "NEXT" => {
                let Some((fv, at)) = self.fors.pop() else {
                    return err(self.ln, "FOR is not exist.");
                };
                let v = if c.done() {
                    fv.clone()
                } else {
                    self.ident(&mut c)?
                };
                if fv != v {
                    return err(self.ln, "Check variable name in FOR and NEXT");
                }
                self.emit(Op::Next { var: v, start: at });
                let h = self.here();
                self.patch(at, h);
            }
            "BREAK" => {
                self.emit(Op::Cmd(first, Vec::new()));
            }
            "GOTO" => {
                let l = self.ident(&mut c)?;
                self.emit(if l == "AUTO" {
                    Op::Cmd("AUTO".into(), Vec::new())
                } else {
                    Op::Goto(l)
                });
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
            "ON" if toks
                .iter()
                .any(|t| matches!(t, Tok::Id(k) if k == "GOTO" || k == "GOSUB")) =>
            {
                let expr = self.expr(&mut c)?;
                let gosub = c.is_id("GOSUB");
                if !c.eat_id("GOTO") && !c.eat_id("GOSUB") {
                    return err(self.ln, "Wrong syntax.");
                }
                let labels = self.label_list(&mut c)?;
                self.emit(Op::On {
                    expr,
                    labels,
                    gosub,
                });
            }
            _ if c.is_op("=") || c.is_op(".") => {
                if self.prog.consts.contains_key(&first) {
                    return err(self.ln, "Can't assign value at constant.");
                }
                if !self.prog.vars.contains_key(&first) {
                    return err(
                        self.ln,
                        "This variable name(or constant name) is not declared.",
                    );
                }
                let bit = if c.eat_op(".") {
                    Some(self.bit(&mut c, &first)?)
                } else {
                    None
                };
                if !c.eat_op("=") {
                    return err(self.ln, "Assegnazione non valida");
                }
                let e = self.expr(&mut c)?;
                self.emit(match bit {
                    Some(bit) => Op::AssignBit(first, bit, e),
                    None => Op::Assign(first, e),
                });
            }
            _ if self.keywords.contains(&first) => {
                let args = if first == "PRINT" {
                    self.print_args(&mut c)?
                } else if first == "AIMOTOR"
                    && ["INIT", "SETON", "SETOFF"].iter().any(|mode| c.is_id(mode))
                {
                    vec![Some(Expr::Var(self.ident(&mut c)?))]
                } else if first == "ERX" {
                    Vec::new()
                } else {
                    self.args(&mut c)?
                };
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
        let cond_end =
            c.t.iter()
                .position(|t| matches!(t, Tok::Id(k) if k == "THEN"));
        let Some(then_at) = cond_end else {
            return err(self.ln, "THEN is not exist.");
        };
        let cond = self.expr(&mut Cur {
            t: &c.t[..then_at],
            p: c.p,
        })?;
        let rest = &c.t[then_at + 1..];
        let jf = self.emit(Op::JmpFalse(cond, 0));
        if rest.is_empty() {
            self.ifs.push(IfFrame {
                pending: Some(jf),
                ends: vec![],
            });
            return Ok(());
        }
        let else_at = rest
            .iter()
            .position(|t| matches!(t, Tok::Id(k) if k == "ELSE"));
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

pub(crate) fn const_eval(e: &Expr, consts: &HashMap<String, i32>) -> Option<i32> {
    match e {
        Expr::Num(n) => Some(*n),
        Expr::Var(v) => consts.get(v).copied(),
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
        "MOD" | "%" => a.checked_rem(b).unwrap_or(0),
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
    let object = crate::compiler::compile(src, "", [0; 6]).map_err(|e| ParseError {
        line: e.line,
        msg: e.msg,
    })?;
    // L'originale produce solo il salto finale se supera l'area delle variabili.
    if object.len() == 20 {
        return Ok(Program::default());
    }
    let mut p = Parser {
        keywords: keywords.iter().cloned().collect(),
        prog: Program::default(),
        ln: 0,
        ifs: Vec::new(),
        fors: Vec::new(),
    };
    for (line, toks) in crate::compiler::expand_poses(src).map_err(|e| ParseError {
        line: e.line,
        msg: e.msg,
    })? {
        p.ln = line;
        let toks = tokens(toks, p.ln)?;
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
    Unsupported,
}

pub struct Interp<'a, H: Host> {
    prog: &'a Program,
    host: &'a mut H,
    pub vars: HashMap<String, i32>,
    pc: usize,
    calls: Vec<usize>,
    loops: HashSet<usize>,
}

impl<'a, H: Host> Interp<'a, H> {
    pub fn new(prog: &'a Program, host: &'a mut H) -> Self {
        let vars = prog.vars.keys().map(|k| (k.clone(), 0)).collect();
        Interp {
            prog,
            host,
            vars,
            pc: 0,
            calls: Vec::new(),
            loops: HashSet::new(),
        }
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
            Expr::Bit(v, bit) => Arg::Int((self.vars[v] >> bit) & 1),
            Expr::Bin(o, a, b) => {
                let (a, b) = (self.eval(a)?, self.eval(b)?);
                Arg::Int(apply(o, a, b).ok_or("Wrong expression.")?)
            }
            Expr::Call(n, args) => {
                if n == "NOT" {
                    return Ok(Arg::Int(!self.eval(&args[0])? & 0xFFFF));
                }
                if n == "FORMAT" {
                    let value = self.eval(&args[0])?;
                    let Expr::Var(mode) = &args[1] else {
                        return Err("Formato non valido".into());
                    };
                    let integer = matches!(&args[0], Expr::Var(v) if self.prog.vars[v] == Ty::Int);
                    let (text, default) = match (mode.as_str(), integer) {
                        ("HEX", false) => (format!("{value:X}"), 2),
                        ("HEX", true) => (format!("{value:X}"), 4),
                        ("DEC", false) => (value.to_string(), 3),
                        ("DEC", true) => (value.to_string(), 5),
                        ("BIN", false) => (format!("{value:b}"), 8),
                        ("BIN", true) => (format!("{value:b}"), 16),
                        _ => return Err("Formato non valido".into()),
                    };
                    let width = if args.len() == 3 {
                        self.eval(&args[2])? as usize
                    } else {
                        default
                    };
                    return Ok(Arg::Str(if text.len() > width {
                        text[..width].into()
                    } else {
                        format!("{text:>width$}")
                    }));
                }
                let args = args
                    .iter()
                    .map(|a| self.arg(a))
                    .collect::<Result<Vec<_>, _>>()?;
                for (i, arg) in args.iter().enumerate() {
                    if let Arg::Int(value) = arg {
                        crate::directives::check_hardware_operand(n, i, *value)?;
                    }
                }
                Arg::Int(self.host.function(n, &args)?)
            }
        })
    }

    fn jump(&self, label: &str) -> usize {
        self.prog.labels[label]
    }

    fn assign(&mut self, var: &str, n: i32) -> i32 {
        let n = match self.prog.vars[var] {
            Ty::Byte => n & 0xFF,
            Ty::Int => n & 0xFFFF,
        };
        self.vars.insert(var.to_string(), n);
        n
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
                    self.assign(v, n);
                }
                Op::AssignBit(v, bit, e) => {
                    let n = self.eval(e).map_err(at)?;
                    if !(0..=1).contains(&n) {
                        return Err(at("Assegnazione di bit: usare 0 o 1".into()));
                    }
                    self.assign(v, (self.vars[v] & !(1 << bit)) | (n << bit));
                }
                Op::Cmd(name, args) => {
                    let mut a = Vec::new();
                    for x in args {
                        a.push(match x {
                            Some(x) => self.arg(x).map_err(at)?,
                            None => Arg::Empty,
                        });
                    }
                    if let Err(msg) = self.host.command(name, &a) {
                        if msg.starts_with("Non simulato:") {
                            return Ok(Stop::Unsupported);
                        }
                        return Err(at(msg));
                    }
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
                Op::On {
                    expr,
                    labels,
                    gosub,
                } => {
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
                Op::For {
                    var,
                    from,
                    to,
                    step,
                    exit,
                } => {
                    let f = self.eval(from).map_err(at)?;
                    let t = self.eval(to).map_err(at)?;
                    let s = match step {
                        Some(s) => self.eval(s).map_err(at)?,
                        None => 1,
                    };
                    let f = self.assign(var, f);
                    self.loops.insert(self.pc);
                    if (s >= 0 && f > t) || (s < 0 && f < t) {
                        next = *exit;
                        self.loops.remove(&self.pc);
                    }
                }
                Op::Next { var, start } => {
                    let Op::For { step, to, .. } = &self.prog.ops[*start] else {
                        unreachable!()
                    };
                    let s = match step {
                        Some(s) => self.eval(s).map_err(at)?,
                        None => 1,
                    };
                    if !self.loops.contains(start) {
                        return Err(at("NEXT raggiunto senza inizializzare FOR".into()));
                    }
                    let t = self.eval(to).map_err(at)?;
                    let v = self.assign(var, self.vars[var].wrapping_add(s));
                    if (s >= 0 && v <= t) || (s < 0 && v >= t) {
                        next = start + 1;
                    } else {
                        self.loops.remove(start);
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
    crate::compiler::group(name)
        .map(|(start, count)| (usize::from(start)..usize::from(start + count)).collect())
        .unwrap_or_default()
}

#[derive(Debug, Serialize)]
pub struct SimRobot {
    pub servos: Vec<u8>,
    pub enabled: Vec<bool>,
    pub speed: i32,
    pub elapsed_ms: i64,
    pub output: Vec<String>,
    pub log: Vec<String>,
    #[serde(skip)]
    pub(crate) settings: crate::directives::Settings,
}

impl Default for SimRobot {
    fn default() -> Self {
        SimRobot {
            servos: vec![100; 32],
            enabled: vec![false; 32],
            speed: 3,
            elapsed_ms: 0,
            output: Vec::new(),
            log: Vec::new(),
            settings: crate::directives::Settings::default(),
        }
    }
}

fn int(a: &Arg) -> Option<i32> {
    if let Arg::Int(n) = a { Some(*n) } else { None }
}

impl Host for SimRobot {
    fn command(&mut self, name: &str, args: &[Arg]) -> Result<(), String> {
        for (i, arg) in args.iter().enumerate() {
            if let Arg::Int(value) = arg {
                crate::directives::check_hardware_operand(name, i, *value)?;
            }
        }
        match name {
            "MOVE" | "MOVE24" => {
                let (motors, vals) = match args.first() {
                    Some(Arg::Group(g)) => (group_motors(g), &args[1..]),
                    _ => ((0..24).collect(), args),
                };
                for (i, m) in motors.iter().enumerate() {
                    if let Some(v) = vals.get(i).and_then(int).filter(|v| *v != 0) {
                        self.settings.check_position(*m, v)?;
                        self.servos[*m] = v as u8;
                    }
                }
            }
            "SERVO" => {
                if let [Arg::Int(m), Arg::Int(v)] = args {
                    self.settings.check_position(*m as usize, *v)?;
                    *self.servos.get_mut(*m as usize).ok_or("Over range.")? = *v as u8;
                }
            }
            "MOTOR" | "MOTOROFF" => {
                let on = name == "MOTOR";
                match args.first() {
                    Some(Arg::Group(g)) => group_motors(g)
                        .into_iter()
                        .for_each(|m| self.enabled[m] = on),
                    Some(Arg::Int(m)) => {
                        *self.enabled.get_mut(*m as usize).ok_or("Over range.")? = on
                    }
                    Some(Arg::Word(mode)) if mode == if on { "ALLON" } else { "ALLOFF" } => {
                        self.enabled.fill(on)
                    }
                    _ => {}
                }
            }
            "SPEED" => self.speed = args.first().and_then(int).unwrap_or(self.speed),
            "DELAY" => self.elapsed_ms += args.first().and_then(int).unwrap_or(0) as i64,
            "PRINT" => self.output.push(
                args.iter()
                    .map(|a| match a {
                        Arg::Int(n) => char::from(*n as u8).to_string(),
                        Arg::Str(s) => s.clone(),
                        _ => String::new(),
                    })
                    .collect::<Vec<_>>()
                    .join(""),
            ),
            "CLS" => self.output.clear(),
            "PTP" | "HIGHSPEED" | "DIR" | "GETMOTORSET" | "LCDINIT" | "LOCATE" | "CSON"
            | "CSOFF" | "WAIT" | "BEEP" | "MUSIC" | "TEMPO" | "GYROSET" | "GYRODIR"
            | "GYROSENSE" | "ACCELSET" | "ACCELDIR" | "ACCELSENSE" | "TILTSET" | "TILTDIR"
            | "TILTSENSE" | "ZERO" | "INIT" | "POS" | "AIMOTOR" | "AIMOTOROFF" | "RUN" => {
                let msg = format!("Effetto hardware non simulato: {name}");
                if !self.log.contains(&msg) {
                    self.log.push(msg);
                }
            }
            "BREAK" => self
                .log
                .push("BREAK non operativo su MR-C3000; non interrompe i cicli FOR".into()),
            "AUTO" => {
                return Err(
                    "GOTO AUTO: template firmware non simulato; esecuzione interrotta".into(),
                );
            }
            _ => {
                let msg = format!("Non simulato: {name}; esecuzione interrotta");
                self.log.push(msg.clone());
                return Err(msg);
            }
        }
        Ok(())
    }

    fn function(&mut self, name: &str, args: &[Arg]) -> Result<i32, String> {
        for (i, arg) in args.iter().enumerate() {
            if let Arg::Int(value) = arg {
                crate::directives::check_hardware_operand(name, i, *value)?;
            }
        }
        if name == "MOTORIN" {
            let m = args.first().and_then(int).ok_or("Motore non valido")?;
            self.log
                .push("MOTORIN: posizione simulata, non letta dal robot".into());
            return self
                .servos
                .get(m as usize)
                .map(|v| i32::from(*v))
                .ok_or("Motore fuori intervallo".into());
        }
        Err(format!(
            "Funzione {name} non simulata: nessun valore hardware disponibile"
        ))
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
        let (v, _) = run(
            "DIM I AS BYTE\nDIM S AS INTEGER\nFOR I = 1 TO 4\n IF I MOD 2 = 0 THEN\n S = S + I\nELSE\n S = S - 1\nENDIF\nNEXT I\nGOSUB X\nEND\nX:\nS = S * 10\nRETURN",
        );
        assert_eq!(v["S"], 40);
    }

    #[test]
    fn block_if_and_on_goto() {
        let (v, _) = run(
            "DIM A AS BYTE\nDIM B AS BYTE\nA = 2\nON A GOTO L1,L2\nL1:\nB = 1\nEND\nL2:\nIF A = 1 THEN\nB = 5\nELSEIF A = 2 THEN\nB = 7\nELSE\nB = 9\nENDIF\nEND",
        );
        assert_eq!(v["B"], 7);
    }

    #[test]
    fn byte_wraps() {
        let (v, _) = run("DIM A AS BYTE\nA = 250 + 10\nEND");
        assert_eq!(v["A"], 4);
    }

    #[test]
    fn move_updates_servos() {
        let (_, sim) = run(
            "DIM A AS BYTE\nMOTOR G6A\nMOVE G6A,10,20,30,40,50,60\nSERVO 23, 77\nDELAY 100\nPRINT \"ok\"",
        );
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
        for src in [
            include_str!("../resources/rrm_header_pc.bas"),
            include_str!("../resources/rrm_header_ir.bas"),
        ] {
            let mut src = src.to_string();
            for n in 1..=32 {
                src.push_str(&format!("\nK{n}:\nGOTO MAIN_LOOP"));
            }
            assert_eq!(
                parse(&src, &kw()).is_ok(),
                crate::compiler::compile(&src, "", [0; 6]).is_ok()
            );
        }
    }

    #[test]
    fn expressions_follow_controller_order() {
        let (vars, _) = run("DIM A AS BYTE\nA = 1 + 2 * 3\nEND");
        assert_eq!(vars["A"], 9);
        for src in [
            "DIM A AS BYTE\nA = NOT A",
            "DIM A AS BYTE\nA = (1 + 2)",
            "DIM A AS BYTE\nFOR A = 1 TO 3 STEP 2\nNEXT A",
        ] {
            assert!(parse(src, &kw()).is_err());
        }
    }

    #[test]
    fn omitted_moves_and_controller_break() {
        let (v, sim) = run("DIM I AS BYTE\nFOR I = 1 TO 3\nBREAK\nNEXT\nMOVE G6A,10,,30\nEND");
        assert_eq!(v["I"], 4);
        assert_eq!(&sim.servos[..6], &[10, 100, 30, 100, 100, 100]);
        let (_, sim) = run("MOVE24 10,,30\nEND");
        assert_eq!(&sim.servos[..4], &[10, 100, 30, 100]);
    }

    #[test]
    fn firmware_auto_never_falls_through_to_a_source_movement() {
        let p = parse("GOTO AUTO\nSERVO 0,10\nEND", &kw()).unwrap();
        let mut sim = SimRobot::default();
        let e = Interp::new(&p, &mut sim).run(10).unwrap_err();
        assert_eq!(e.line, 1);
        assert!(e.msg.contains("template firmware non simulato"));
        assert_eq!(sim.servos[0], 100);
        let (_, sim) = run("IF 1=0 THEN GOTO AUTO\nSERVO 0,10\nEND");
        assert_eq!(sim.servos[0], 10);
    }

    #[test]
    fn dynamic_hardware_operands_are_checked_before_simulated_effects() {
        for (value, command) in [
            (255, "SERVO 0,A"),
            (24, "SERVO A,100"),
            (24, "MOTOR A"),
            (24, "A=MOTORIN(A)"),
            (8, "A=AD(A)"),
            (16, "SPEED A"),
        ] {
            let src = format!("DIM A AS BYTE\nA={value}\n{command}\nEND");
            let p = parse(&src, &kw()).unwrap();
            let mut sim = SimRobot::default();
            let e = Interp::new(&p, &mut sim).run(10).unwrap_err();
            assert_eq!(e.line, 3, "{src}");
            assert!(e.msg.contains("fuori intervallo"), "{e:?}");
            assert!(sim.servos.iter().all(|position| *position == 100));
            assert!(sim.enabled.iter().all(|enabled| !enabled));
        }
    }

    #[test]
    fn hardware_is_never_silently_zero() {
        let p = parse("DIM A AS BYTE\nA = AD(0)\nEND", &kw()).unwrap();
        let mut sim = SimRobot::default();
        assert!(
            Interp::new(&p, &mut sim)
                .run(10)
                .unwrap_err()
                .msg
                .contains("non simulata")
        );
        let p = parse("STOP\nSERVO 0,20\nEND", &kw()).unwrap();
        assert_eq!(Interp::new(&p, &mut sim).run(10), Ok(Stop::Unsupported));
        assert_eq!(sim.servos[0], 100);
        assert!(!sim.log.is_empty());
    }

    #[test]
    fn parses_every_compilable_sample() {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../re/samples");
        let mut failures = Vec::new();
        for entry in std::fs::read_dir(dir).unwrap() {
            let p = entry.unwrap().path();
            if p.extension().and_then(|x| x.to_str()) != Some("hex") {
                continue;
            }
            let src = std::fs::read_to_string(p.with_extension("bas")).unwrap();
            if let Err(e) = parse(&src, &kw()) {
                failures.push(format!("{}: {e:?}", p.display()));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn variable_overflow_simulates_empty_object() {
        let src = (0..193)
            .map(|n| format!("DIM V{n} AS BYTE\n"))
            .collect::<String>()
            + "SERVO 0,20\nEND";
        let (_, sim) = run(&src);
        assert_eq!(sim.servos[0], 100);
    }

    #[test]
    fn loops_preserve_byte_width_and_reject_uninitialized_entry() {
        let p = parse("DIM I AS BYTE\nFOR I = 254 TO 255\nNEXT I\nEND", &kw()).unwrap();
        let mut sim = SimRobot::default();
        let mut i = Interp::new(&p, &mut sim);
        assert_eq!(i.run(10), Ok(Stop::StepLimit));
        assert!(i.vars["I"] <= 255);
        let p = parse(
            "DIM I AS BYTE\nGOTO INSIDE\nFOR I = 0 TO 3\nINSIDE:\nNEXT I\nEND",
            &kw(),
        )
        .unwrap();
        assert!(
            Interp::new(&p, &mut sim)
                .run(10)
                .unwrap_err()
                .msg
                .contains("FOR")
        );
        let (v, _) = run("REMARK commento\nDIM I AS BYTE\nFOR I = 256 TO 2\nNEXT I\nEND");
        assert_eq!(v["I"], 3);
    }

    #[test]
    fn unsigned_words_bits_and_not() {
        let (v, _) = run(
            "DIM A AS INTEGER\nDIM B AS BYTE\nCONST TOP = 15\nA = 65535\nA.TOP = 0\nB.7 = A.0\nA = NOT(A)\nB = NOT(B)\nEND",
        );
        assert_eq!(v["A"], 32768);
        assert_eq!(v["B"], 127);
        let (v, _) = run("DIM A AS INTEGER\nA = 65535\nA = A + 1\nA = A - 1\nEND");
        assert_eq!(v["A"], 65535);
    }

    #[test]
    fn operator_classes_keep_arithmetic_sequential() {
        let (v, _) = run(
            "DIM A AS BYTE\nDIM B AS BYTE\nDIM R AS BYTE\nA = 1\nB = 2\nIF A = 1 AND B = 2 THEN R = 1\nIF A = 0 OR B = 2 THEN R = R + 2\nIF A = 1 XOR B = 0 THEN R = R + 4\nA = 1 + 2 * 3\nB = &H3D + &B10 * &O3 % 10\nEND",
        );
        assert_eq!(v["R"], 7);
        assert_eq!(v["A"], 9);
        assert_eq!(v["B"], 9);
        let (v, _) = run("DIM R AS BYTE\nIF 1 XOR 2 = 3 THEN R = 1\nEND");
        assert_eq!(v["R"], 1);
        let (v, _) = run(
            "DIM R AS BYTE\nIF 2 > 1 AND 3 XOR 2 = 3 THEN R = 1\nIF 2 > 1 AND 1 XOR 2 = 3 THEN R = R + 2\nEND",
        );
        assert_eq!(v["R"], 2);
    }

    #[test]
    fn constants_are_bound_where_the_compiler_emits_them() {
        let (v, _) =
            run("CONST P = 150\nDIM A AS BYTE\nDIM B AS BYTE\nA = P\nCONST P = 50\nB = P\nEND");
        assert_eq!(v["A"], 150);
        assert_eq!(v["B"], 50);
    }

    #[test]
    fn lcd_characters_and_formats() {
        let (v, sim) = run(
            "DIM A AS BYTE\nDIM B AS INTEGER\nA = \"AB\"\nB = &HABCD\nPRINT \"OK\",65,13,10\nPRINT FORMAT(A,DEC,4)\nPRINT FORMAT(B,HEX)\nPRINT FORMAT(B,BIN,4)\nEND",
        );
        assert_eq!(v["A"], 65);
        assert_eq!(sim.output, ["OKA\r\n", "  65", "ABCD", "1010"]);
    }

    #[test]
    fn named_poses_move_only_when_called() {
        let (_, sim) = run(
            "MOVEPOS P1\nSERVO 0,80\nIF 1=1 THEN MOVEPOS P2\nIF 1=1 THEN IF 1=0 THEN MOVEPOS P1\nEND\nP1: POS G6A,110,,120\nREMARK gruppo successivo\nPOS G6B,90,100,110\nP2:\nPOS G6A,,70\n",
        );
        assert_eq!(&sim.servos[..3], &[80, 70, 120]);
        assert_eq!(&sim.servos[6..9], &[90, 100, 110]);
        let (_, sim) = run("POS G6A,110,120\nZERO G6A,100,100\nINIT G6A,100,100\nRUN\nRUN\nEND");
        assert_eq!(sim.servos[0], 100);
        assert!(sim.log.iter().any(|msg| msg.contains("INIT")));
        assert_eq!(
            sim.log.iter().filter(|msg| msg.ends_with(": RUN")).count(),
            1
        );
    }

    #[test]
    fn simulator_never_moves_outside_declared_limits() {
        let mut sim = SimRobot {
            settings: crate::directives::parse("'$LIMIT 0,50,100").unwrap(),
            ..SimRobot::default()
        };
        sim.command("SERVO", &[Arg::Int(0), Arg::Int(50)]).unwrap();
        assert!(sim.command("SERVO", &[Arg::Int(0), Arg::Int(150)]).is_err());
        assert_eq!(sim.servos[0], 50);
        assert!(
            sim.command("MOVE", &[Arg::Group("G6A".into()), Arg::Int(150)])
                .is_err()
        );
        assert_eq!(sim.servos[0], 50);
    }

    #[test]
    fn keyin_never_invents_a_sensor_value() {
        let p = parse("DIM K AS BYTE\nK = KEYIN(0,16)\nEND", &kw()).unwrap();
        let mut sim = SimRobot::default();
        let e = Interp::new(&p, &mut sim).run(10).unwrap_err();
        assert_eq!(e.line, 2);
        assert!(e.msg.contains("KEYIN") && e.msg.contains("non simulata"));
    }
}
