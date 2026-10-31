//! Compilatore roboBASIC -> binario per il firmware (MR-C3000), ricavato per
//! black box dal compilatore originale (`rbv2.ocx`). Vedi `re/NOTES.md`.
//!
//! Copre il sottoinsieme verificato byte per byte con `re/samples`; i comandi
//! non ancora decodificati restituiscono `CompileError::unsupported`.

use std::collections::HashMap;

const BODY_BASE: usize = 17;

#[derive(Debug, Clone, PartialEq)]
pub struct CompileError {
    pub line: usize,
    pub msg: String,
    pub unsupported: bool,
}

type Res<T> = Result<T, CompileError>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tok {
    Id(String),
    Num(i64),
    Str(String),
    Op(String),
}

#[derive(Clone, Copy)]
struct Var {
    int: bool,
    addr: u8,
}

struct IfFrame {
    false_fix: Option<usize>,
    end_fixes: Vec<usize>,
    inline: bool,
}

struct ForFrame {
    var: Var,
    check_addr: u16,
    exit_fix: usize,
}

struct Cur {
    toks: Vec<Tok>,
    pos: usize,
}

impl Cur {
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }
    fn next(&mut self) -> Option<Tok> {
        let t = self.toks.get(self.pos).cloned();
        self.pos += 1;
        t
    }
    fn done(&self) -> bool {
        self.pos >= self.toks.len()
    }
    fn is_id(&self, k: &str) -> bool {
        matches!(self.peek(), Some(Tok::Id(s)) if s == k)
    }
    fn eat_id(&mut self, k: &str) -> bool {
        let ok = self.is_id(k);
        if ok {
            self.pos += 1;
        }
        ok
    }
    fn is_op(&self, o: &str) -> bool {
        matches!(self.peek(), Some(Tok::Op(s)) if s == o)
    }
    fn eat_op(&mut self, o: &str) -> bool {
        let ok = self.is_op(o);
        if ok {
            self.pos += 1;
        }
        ok
    }
}

pub(crate) fn lex(line: &str) -> Vec<Tok> {
    let cs: Vec<char> = line.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '\'' {
            break;
        } else if c == '"' {
            let mut s = String::new();
            i += 1;
            while i < cs.len() && cs[i] != '"' {
                s.push(cs[i]);
                i += 1;
            }
            i += 1;
            out.push(Tok::Str(s));
        } else if c == '&'
            && i + 1 < cs.len()
            && matches!(cs[i + 1].to_ascii_uppercase(), 'H' | 'B' | 'O')
        {
            let radix = match cs[i + 1].to_ascii_uppercase() {
                'H' => 16,
                'B' => 2,
                _ => 8,
            };
            i += 2;
            let st = i;
            while i < cs.len() && cs[i].is_ascii_alphanumeric() {
                i += 1;
            }
            let s: String = cs[st..i].iter().collect();
            out.push(Tok::Num(i64::from_str_radix(&s, radix).unwrap_or(i64::MAX)));
        } else if c.is_ascii_digit() {
            let st = i;
            while i < cs.len() && cs[i].is_ascii_digit() {
                i += 1;
            }
            let s: String = cs[st..i].iter().collect();
            out.push(Tok::Num(s.parse().unwrap_or(i64::MAX)));
        } else if c.is_alphabetic() || c == '_' {
            let st = i;
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            let s: String = cs[st..i].iter().collect();
            out.push(Tok::Id(s.to_uppercase()));
        } else {
            let two: String = cs[i..(i + 2).min(cs.len())].iter().collect();
            if ["<=", ">=", "<>", "<<", ">>"].contains(&two.as_str()) {
                out.push(Tok::Op(two));
                i += 2;
            } else {
                out.push(Tok::Op(c.to_string()));
                i += 1;
            }
        }
    }
    out
}

pub(crate) fn group(name: &str) -> Option<(u8, u8)> {
    Some(match name {
        "G6A" => (0, 6),
        "G6B" => (6, 6),
        "G6C" => (12, 6),
        "G6D" => (18, 6),
        "G6E" => (24, 6),
        "G8A" => (0, 8),
        "G8B" => (8, 8),
        "G8C" => (16, 8),
        "G8D" => (24, 8),
        "G12" => (0, 12),
        "G16" => (0, 16),
        "G24" => (0, 24),
        "G32" => (0, 32),
        _ => return None,
    })
}

fn binop(op: &str) -> Option<u8> {
    Some(match op {
        "+" => 0x21,
        "-" => 0x22,
        "*" => 0x23,
        "/" => 0x24,
        "MOD" | "%" => 0x26,
        "AND" => 0x27,
        "OR" => 0x28,
        "XOR" => 0x2A,
        "<<" => 0x2C,
        ">>" => 0x2D,
        "<" => 0x30,
        ">" => 0x31,
        "<=" => 0x32,
        ">=" => 0x33,
        "<>" => 0x34,
        "=" => 0x35,
        _ => return None,
    })
}

pub(crate) fn func_code(name: &str) -> Option<(u8, bool)> {
    Some(match name {
        "RND" => (0xF8, false),
        "IN" => (0xF0, true),
        "KEYIN" => (0xF1, true),
        "NOT" => (0xEF, true),
        "BYTEIN" => (0xF2, true),
        "AD" => (0xF3, true),
        "PEEK" => (0xF9, true),
        "REMOCON" => (0xFB, true),
        "SONAR" => (0xFD, true),
        "ROMPEEK" => (0xFA, true),
        "MOTORIN" => (0xFE, true),
        "STATE" => (0xF6, true),
        "AIMOTORIN" => (0xFC, true),
        _ => return None,
    })
}

// L'OCX attuale rifiuta MOVEPOS: abbassarlo a MOVE usa solo bytecode verificato.
pub(crate) fn expand_poses(src: &str) -> Res<Vec<(usize, Vec<Tok>)>> {
    let mut lines = Vec::new();
    for (i, raw) in src.lines().enumerate() {
        let toks = lex(raw);
        if toks.len() > 2
            && matches!(&toks[0], Tok::Id(_))
            && toks[1] == Tok::Op(":".into())
            && toks[2] == Tok::Id("POS".into())
        {
            lines.push((i + 1, toks[..2].to_vec()));
            lines.push((i + 1, toks[2..].to_vec()));
        } else if !toks.is_empty() {
            lines.push((i + 1, toks));
        }
    }
    let mut poses: HashMap<String, Vec<Vec<Tok>>> = HashMap::new();
    let mut labels = Vec::new();
    let mut after_pos = false;
    for (_, toks) in &lines {
        if toks.first() == Some(&Tok::Id("REMARK".into()))
            && !matches!(toks.get(1), Some(Tok::Op(op)) if op == "=" || op == ".")
        {
            continue;
        }
        if let [Tok::Id(label), Tok::Op(colon)] = toks.as_slice()
            && colon == ":"
        {
            if after_pos {
                labels.clear();
            }
            labels.push(label.clone());
            after_pos = false;
            continue;
        }
        if toks.first() == Some(&Tok::Id("POS".into())) && matches!(toks.get(1), Some(Tok::Id(_))) {
            for label in &labels {
                poses.entry(label.clone()).or_default().push(toks.clone());
            }
            after_pos = true;
        } else {
            labels.clear();
            after_pos = false;
        }
    }
    let mut expanded = Vec::new();
    for (line, toks) in lines {
        let move_at = if toks.first() == Some(&Tok::Id("MOVEPOS".into())) {
            Some(0)
        } else if toks.first() == Some(&Tok::Id("IF".into())) {
            toks.windows(2)
                .rposition(|pair| pair == [Tok::Id("THEN".into()), Tok::Id("MOVEPOS".into())])
                .map(|at| at + 1)
        } else {
            None
        }.filter(|at| !matches!(toks.get(at + 1), Some(Tok::Op(op)) if ["=", ".", ":"].contains(&op.as_str())));
        if let Some(move_at) = move_at {
            let pose = match &toks[move_at..] {
                [_, Tok::Id(label)] => poses.get(label),
                _ => None,
            };
            let Some(pose) = pose else {
                return Err(CompileError {
                    line,
                    msg: "MOVEPOS richiede un'etichetta che definisce una posa POS".into(),
                    unsupported: false,
                });
            };
            let mut start = 0;
            let mut conditions = 0;
            while start < move_at {
                let then = toks[start..move_at]
                    .iter()
                    .position(|tok| tok == &Tok::Id("THEN".into()))
                    .unwrap()
                    + start;
                expanded.push((line, toks[start..=then].to_vec()));
                conditions += 1;
                start = then + 1;
            }
            for block in pose {
                let mut movement = block.clone();
                movement[0] = Tok::Id("MOVE".into());
                expanded.push((line, movement));
            }
            for _ in 0..conditions {
                expanded.push((line, vec![Tok::Id("ENDIF".into())]));
            }
        } else {
            expanded.push((line, toks));
        }
    }
    Ok(expanded)
}

struct Compiler {
    out: Vec<u8>,
    line: usize,
    vars: HashMap<String, Var>,
    consts: HashMap<String, i64>,
    next_var: usize,
    labels: HashMap<String, u16>,
    fixups: Vec<(usize, String, usize)>,
    ifs: Vec<IfFrame>,
    fors: Vec<ForFrame>,
}

impl Compiler {
    fn err<T>(&self, msg: &str) -> Res<T> {
        Err(CompileError {
            line: self.line,
            msg: msg.to_string(),
            unsupported: false,
        })
    }
    fn unsupported<T>(&self, what: &str) -> Res<T> {
        Err(CompileError {
            line: self.line,
            msg: format!("{what}: non supportato"),
            unsupported: true,
        })
    }

    fn here(&self) -> u16 {
        (BODY_BASE - 1 + self.out.len()) as u16
    }

    fn put_addr(&mut self, a: u16) {
        self.out.extend_from_slice(&a.to_le_bytes());
    }

    fn placeholder(&mut self) -> usize {
        let at = self.out.len();
        self.out.extend_from_slice(&[0, 0]);
        at
    }

    fn patch_here(&mut self, at: usize) {
        let h = self.here().to_le_bytes();
        self.out[at] = h[0];
        self.out[at + 1] = h[1];
    }

    fn jump_label(&mut self, label: &str) {
        let at = self.placeholder();
        self.fixups.push((at, label.to_string(), self.line));
    }

    fn num(&mut self, v: i64) -> Res<()> {
        match v {
            0 => self.out.push(0x10),
            1 => self.out.push(0x11),
            2..=255 => self.out.extend_from_slice(&[0x12, v as u8]),
            256..=65535 => {
                self.out.push(0x13);
                self.out.extend_from_slice(&(v as u16).to_le_bytes());
            }
            _ => return self.err("numero fuori intervallo"),
        }
        Ok(())
    }

    fn const_num(&self, c: &mut Cur) -> Res<Option<i64>> {
        match c.peek() {
            Some(Tok::Num(n)) => {
                let n = *n;
                c.pos += 1;
                Ok(Some(n))
            }
            Some(Tok::Id(s)) if self.consts.contains_key(s) => {
                let v = self.consts[s];
                c.pos += 1;
                Ok(Some(v))
            }
            _ => Ok(None),
        }
    }

    fn var(&self, name: &str) -> Res<Var> {
        match self.vars.get(name) {
            Some(v) => Ok(*v),
            None => self.err("선언되어 있지 않은 변수명(상수명)이 사용되었습니다."),
        }
    }

    fn put_var(&mut self, v: Var) {
        self.out
            .extend_from_slice(&[if v.int { 0x16 } else { 0x15 }, v.addr]);
    }

    fn put_reference(&mut self, c: &mut Cur, v: Var) -> Res<()> {
        if c.eat_op(".") {
            let Some(bit @ 0..=15) = self.const_num(c)? else {
                return self.err("Indicatore di bit fuori intervallo");
            };
            if !v.int && bit > 7 {
                return self.err("Indicatore di bit BYTE fuori intervallo (0-7)");
            }
            self.out.extend_from_slice(&[
                0x1B,
                v.addr.wrapping_add((bit / 8) as u8),
                1 << (bit % 8),
            ]);
        } else {
            self.put_var(v);
        }
        Ok(())
    }

    fn operand(&mut self, c: &mut Cur) -> Res<()> {
        if let Some(n) = self.const_num(c)? {
            return self.num(n);
        }
        match c.next() {
            Some(Tok::Id(s)) => {
                if s == "RCIN" {
                    // RCIN(n) è un AD sul canale 100+n
                    if !c.eat_op("(") {
                        return self.err("형식이 맞지 않습니다.");
                    }
                    let Some(n @ 0..=3) = self.const_num(c)? else {
                        return self.err("형식이 맞지 않습니다.");
                    };
                    if !c.eat_op(")") {
                        return self.err("형식이 맞지 않습니다.");
                    }
                    self.out.push(0xF3);
                    return self.num(100 + n);
                }
                if let Some((code, has_arg)) = func_code(&s) {
                    self.out.push(code);
                    if has_arg {
                        if !c.eat_op("(") {
                            return self.err("형식이 맞지 않습니다.");
                        }
                        self.operand(c)?;
                        if s == "KEYIN" {
                            self.comma(c)?;
                            self.operand(c)?;
                        }
                        if !c.eat_op(")") {
                            return self.err("형식이 맞지 않습니다.");
                        }
                    }
                    return Ok(());
                }
                let v = self.var(&s)?;
                if c.is_op("(") {
                    return self.unsupported("array");
                }
                self.put_reference(c, v)
            }
            Some(Tok::Str(s)) if s.is_ascii() && !s.is_empty() => self.num(s.as_bytes()[0] as i64),
            _ => self.err("형식이 맞지 않습니다."),
        }
    }

    fn expr(&mut self, c: &mut Cur) -> Res<()> {
        self.operand(c)?;
        loop {
            let op = match c.peek() {
                Some(Tok::Op(o)) => o.clone(),
                Some(Tok::Id(o)) if ["MOD", "AND", "OR", "XOR"].contains(&o.as_str()) => o.clone(),
                _ => break,
            };
            let Some(code) = binop(&op) else { break };
            c.pos += 1;
            self.out.push(code);
            self.operand(c)?;
        }
        self.out.push(0x00);
        Ok(())
    }

    fn comma(&self, c: &mut Cur) -> Res<()> {
        if c.eat_op(",") {
            Ok(())
        } else {
            self.err("형식이 맞지 않습니다.")
        }
    }

    fn ops(&mut self, c: &mut Cur, n: usize) -> Res<()> {
        for i in 0..n {
            if i > 0 {
                self.comma(c)?;
            }
            self.operand(c)?;
        }
        Ok(())
    }

    fn label_name(&self, c: &mut Cur) -> Res<String> {
        match c.next() {
            Some(Tok::Id(s)) => Ok(s),
            _ => self.err("형식이 맞지 않습니다."),
        }
    }

    fn group_arg(&self, c: &mut Cur) -> Res<(u8, u8)> {
        match c.next() {
            Some(Tok::Id(s)) => match group(&s) {
                Some(g) => Ok(g),
                None if s.starts_with('G') => self.unsupported("gruppo"),
                None => self.err("형식이 맞지 않습니다."),
            },
            _ => self.err("형식이 맞지 않습니다."),
        }
    }

    fn values(&self, c: &mut Cur, count: usize) -> Res<Vec<u8>> {
        self.values_range(c, count, 0..=255)
    }

    fn values_range(
        &self,
        c: &mut Cur,
        count: usize,
        range: std::ops::RangeInclusive<i64>,
    ) -> Res<Vec<u8>> {
        let mut v = vec![0u8; count];
        for slot in v.iter_mut() {
            if c.done() {
                break;
            }
            if !c.is_op(",") {
                match self.const_num(c)? {
                    Some(n) if range.contains(&n) => *slot = n as u8,
                    Some(_) => return self.err("범위 초과"),
                    None => return self.err("[MOVE(동작)] 숫자 또는 상수만 사용할 수 있습니다."),
                }
            }
            if !c.eat_op(",") {
                break;
            }
        }
        if !c.done() {
            return self.err("형식이 맞지 않습니다.");
        }
        Ok(v)
    }

    fn motor_cmd(&mut self, c: &mut Cur, opcode: u8) -> Res<()> {
        if let Some(Tok::Id(s)) = c.peek() {
            if let Some((start, count)) = group(s).or_else(|| {
                let mode = if matches!(opcode, 0xDB | 0xB8) {
                    "ALLOFF"
                } else {
                    "ALLON"
                };
                let count = if matches!(opcode, 0xB7 | 0xB8) {
                    31
                } else {
                    32
                };
                (s == mode).then_some((0, count))
            }) {
                c.pos += 1;
                for m in start..start + count {
                    self.out.push(opcode);
                    self.num(m as i64)?;
                }
                return Ok(());
            }
            if s.starts_with('G') && self.vars.get(s).is_none() {
                return self.unsupported("gruppo");
            }
        }
        self.out.push(opcode);
        self.operand(c)
    }

    fn statement(&mut self, c: &mut Cur) -> Res<()> {
        let Some(Tok::Id(kw)) = c.next() else {
            return self.err("구문처리 에러가 발생했습니다.");
        };
        if c.is_op("=") || c.is_op(".") {
            let v = self.var(&kw)?;
            self.out.push(0xD0);
            self.put_reference(c, v)?;
            if !c.eat_op("=") {
                return self.err("형식이 맞지 않습니다.");
            }
            return self.expr(c);
        }
        match kw.as_str() {
            "END" => {
                let h = self.here();
                self.out.push(0xC4);
                self.put_addr(h);
            }
            "GOTO" => {
                let l = self.label_name(c)?;
                if l == "AUTO" {
                    self.out
                        .extend_from_slice(&[0xD0, 0x15, 0x40, 0x10, 0x00, 0xC4, 0x28, 0x27]);
                } else {
                    self.out.push(0xC4);
                    self.jump_label(&l);
                }
            }
            "GOSUB" => {
                let l = self.label_name(c)?;
                self.out.push(0xC5);
                self.jump_label(&l);
            }
            "RETURN" => self.out.push(0xC6),
            "ON" => {
                let mut tmp = Vec::new();
                std::mem::swap(&mut tmp, &mut self.out);
                let r = self.operand(c);
                std::mem::swap(&mut tmp, &mut self.out);
                r?;
                if !c.eat_id("GOTO") {
                    return self.unsupported("ON..GOSUB");
                }
                let mut labels = vec![self.label_name(c)?];
                while c.eat_op(",") {
                    labels.push(self.label_name(c)?);
                }
                self.out.push(0xC7);
                self.out.extend_from_slice(&tmp);
                self.out.push(labels.len() as u8);
                for l in labels {
                    self.jump_label(&l);
                }
            }
            "DIM" => {
                let Some(Tok::Id(name)) = c.next() else {
                    return self.err("[DIM(선언)] 변수(상수) 선언이 잘못되었습니다.");
                };
                if c.is_op("(") || !c.eat_id("AS") {
                    return self.err("[DIM(선언)] 변수(상수) 선언이 잘못되었습니다.");
                }
                let int = if c.eat_id("INTEGER") {
                    true
                } else if c.eat_id("BYTE") {
                    false
                } else {
                    return self.err("[DIM(선언)] 변수(상수) 선언이 잘못되었습니다.");
                };
                let size = if int { 2 } else { 1 };
                if self.next_var + size > 0x100 {
                    return self.err("변수 영역 초과");
                }
                self.vars.insert(
                    name,
                    Var {
                        int,
                        addr: self.next_var as u8,
                    },
                );
                self.next_var += size;
            }
            "CONST" => {
                let Some(Tok::Id(name)) = c.next() else {
                    return self.err("[CONST] 형식이 맞지 않습니다.");
                };
                if !c.eat_op("=") {
                    return self.err("[CONST] 형식이 맞지 않습니다.");
                }
                match self.const_num(c)? {
                    Some(v) => {
                        self.consts.insert(name, v);
                    }
                    None => return self.err("[CONST] 형식이 맞지 않습니다."),
                }
            }
            "DELAY" => {
                self.out.push(0xD4);
                self.operand(c)?;
                self.out.push(0xE9);
            }
            "SPEED" => {
                self.out.push(0xDD);
                self.operand(c)?;
            }
            "SERVO" => {
                self.out.push(0xDF);
                self.ops(c, 2)?;
            }
            "OUT" => {
                self.out.push(0xD1);
                self.ops(c, 2)?;
            }
            "TOGGLE" => {
                self.out.push(0xD3);
                self.operand(c)?;
            }
            "PULSE" => {
                self.out.push(0xD2);
                self.operand(c)?;
            }
            "BYTEOUT" => {
                self.out.push(0xE7);
                self.ops(c, 2)?;
            }
            "POKE" => {
                self.out.push(0xCC);
                self.ops(c, 2)?;
            }
            "CONT" => {
                self.out.push(0xE6);
                self.operand(c)?;
            }
            "LOCATE" => {
                self.out.push(0xE2);
                self.ops(c, 2)?;
            }
            "TEMPO" => {
                self.out.push(0xBF);
                self.operand(c)?;
            }
            "LCDINIT" => self.out.push(0xE0),
            "CLS" => self.out.push(0xE1),
            "CSON" => self.out.push(0xE4),
            "CSOFF" => self.out.push(0xE5),
            "WAIT" => self.out.push(0xE9),
            "STOP" => self.out.extend_from_slice(&[0xEA, 0xE9]),
            "RUN" => self.out.push(0xEB),
            "PRINT" => self.print(c)?,
            "MOTOR" => self.motor_cmd(c, 0xDA)?,
            "MOTOROFF" => self.motor_cmd(c, 0xDB)?,
            "MOVE" | "MOVE24" | "POS" | "ZERO" | "INIT" => {
                let (start, count) = if kw == "MOVE24" {
                    (0, 24)
                } else {
                    self.group_arg(c)?
                };
                if kw != "MOVE24" {
                    self.comma(c)?;
                }
                let range = if kw == "ZERO" { 80..=120 } else { 10..=190 };
                let vals = self.values_range(c, count as usize, range)?;
                let opcode = match kw.as_str() {
                    "POS" => 0xB1,
                    "ZERO" => 0xB3,
                    "INIT" => 0xB5,
                    _ => 0xB0,
                };
                self.out.extend_from_slice(&[opcode, start, count]);
                self.out.extend_from_slice(&vals);
            }
            "DIR" => {
                let (start, count) = self.group_arg(c)?;
                if count > 8 {
                    return self.unsupported("DIR su gruppi > 8");
                }
                self.comma(c)?;
                let vals = self.values(c, count as usize)?;
                let mask = vals
                    .iter()
                    .enumerate()
                    .fold(0u8, |m, (i, v)| m | ((*v & 1) << i));
                self.out.extend_from_slice(&[0xB4, start, count, mask]);
            }
            "GETMOTORSET" => {
                let (start, count) = self.group_arg(c)?;
                self.comma(c)?;
                let vals = self.values(c, count as usize)?;
                self.out.extend_from_slice(&[0xB9, start, count]);
                self.out.extend_from_slice(&vals);
            }
            "PTP" | "HIGHSPEED" => {
                let Some(Tok::Id(m)) = c.next() else {
                    return self.err("형식이 맞지 않습니다.");
                };
                let bytes: [u8; 2] = match (kw.as_str(), m.as_str()) {
                    ("PTP", "SETON") => [0xED, 1],
                    ("PTP", "SETOFF") => [0xED, 0],
                    ("PTP", "ALLON") => [0xB6, 6],
                    ("PTP", "ALLOFF") => [0xB6, 5],
                    ("HIGHSPEED", "SETON") => [0xB6, 1],
                    ("HIGHSPEED", "SETOFF") => [0xB6, 0],
                    _ => return self.err("형식이 맞지 않습니다."),
                };
                self.out.extend_from_slice(&bytes);
            }
            "FOR" => {
                let Some(Tok::Id(name)) = c.next() else {
                    return self.err("[FOR(반복)] 명령어의 형식이 맞지 않습니다.");
                };
                let v = self.var(&name)?;
                if !c.eat_op("=") {
                    return self.err("[FOR(반복)] 명령어의 형식이 맞지 않습니다.");
                }
                self.out.push(0xC1);
                self.put_var(v);
                self.operand(c)?;
                if !c.eat_id("TO") {
                    return self.err("[FOR(반복)] 명령어의 형식이 맞지 않습니다.");
                }
                let check_addr = self.here();
                self.out.push(0xC3);
                self.put_var(v);
                self.operand(c)?;
                let exit_fix = self.placeholder();
                if c.is_id("STEP") {
                    return self.err("[FOR(반복)] 명령어의 형식이 맞지 않습니다.");
                }
                self.fors.push(ForFrame {
                    var: v,
                    check_addr,
                    exit_fix,
                });
            }
            "NEXT" => {
                let Some(f) = self.fors.pop() else {
                    return self.err("[NEXT] FOR 없이 사용되었습니다.");
                };
                if let Some(Tok::Id(n)) = c.peek() {
                    if self.var(n)?.addr != f.var.addr {
                        return self.err("[NEXT] 변수가 일치하지 않습니다.");
                    }
                    c.pos += 1;
                }
                self.out.push(0xC2);
                self.put_var(f.var);
                self.put_addr(f.check_addr);
                self.patch_here(f.exit_fix);
            }
            "BREAK" => self.out.push(0xCB),
            "BEEP" => self.out.extend_from_slice(&[0xCF, 0x14, 0x02, 0x00]),
            "PWM" => {
                self.out.push(0xDE);
                self.ops(c, 2)?;
            }
            "FPWM" => {
                self.out.push(0xEE);
                self.ops(c, 3)?;
            }
            "ROMPOKE" => {
                self.out.push(0xCD);
                self.ops(c, 2)?;
            }
            "AIMOTOR" => {
                let mode = match c.peek() {
                    Some(Tok::Id(s)) => match s.as_str() {
                        "SETOFF" => Some(2),
                        "SETON" => Some(3),
                        "INIT" => Some(4),
                        _ => None,
                    },
                    _ => None,
                };
                if let Some(mode) = mode {
                    c.pos += 1;
                    self.out.extend_from_slice(&[0xB6, mode]);
                } else {
                    self.motor_cmd(c, 0xB7)?;
                }
            }
            "AIMOTOROFF" => {
                self.motor_cmd(c, 0xB8)?;
            }
            "FUNC" => match self.const_num(c)? {
                Some(n @ 0..=255) => self.out.extend_from_slice(&[0xB6, n as u8]),
                _ => return self.err("[FUNC] 숫자 또는 상수만 사용할 수 있습니다."),
            },
            "ACTION" => match self.const_num(c)? {
                Some(n) if (0..=200).contains(&n) => {
                    self.out.extend_from_slice(&[0xD0, 0x15, 0x40]);
                    self.num(0x32 + n)?;
                    self.out.extend_from_slice(&[0x00, 0xC5, 0x28, 0x27]);
                }
                _ => return self.err("[ACTION(행동)] 숫자 또는 상수만 사용할 수 있습니다."),
            },
            "GYROSET" | "GYRODIR" | "GYROSENSE" => {
                let (start, count) = self.group_arg(c)?;
                self.comma(c)?;
                let vals = self.values(c, count as usize)?;
                let op = match kw.as_str() {
                    "GYROSET" => 0xA0,
                    "GYRODIR" => 0xA1,
                    _ => 0xA2,
                };
                self.out.extend_from_slice(&[op, start, count]);
                self.out.extend_from_slice(&vals);
            }
            "ACCELDIR" | "ACCELSENSE" | "TILTDIR" | "TILTSENSE" => {
                self.group_arg(c)?;
                self.comma(c)?;
                let vals = self.values(c, 6)?;
                if kw.ends_with("DIR") && vals.iter().any(|v| *v > 1) {
                    return self.err("사용할 수 있는 숫자의 범위를 벗어났습니다.");
                }
                self.out.extend_from_slice(&[0x00, 0x20]);
                self.out.extend_from_slice(&vals);
                self.out.extend_from_slice(&[0; 26]);
            }
            "ACCELSET" | "TILTSET" => {
                if self.const_num(c)? != Some(1) {
                    return self.unsupported(&kw);
                }
                let mut vals = vec![0u8; 32];
                let mut i = 0;
                while c.eat_op(",") {
                    match self.const_num(c)? {
                        Some(n @ 0..=255) if i < 32 => vals[i] = n as u8,
                        _ => return self.err("형식이 맞지 않습니다."),
                    }
                    i += 1;
                }
                self.out.extend_from_slice(&[
                    if kw == "ACCELSET" { 0xA0 } else { 0xA9 },
                    0x00,
                    0x20,
                ]);
                self.out.extend_from_slice(&vals);
            }
            "ETX" | "ERX" => {
                let baud = match self.const_num(c)? {
                    Some(2400) => 0,
                    Some(4800) => 1,
                    Some(9600) => 2,
                    Some(14400) => 3,
                    Some(19200) => 4,
                    Some(28800) => 5,
                    Some(38400) => 6,
                    Some(57600) => 7,
                    Some(76800) => 8,
                    Some(115200) => 9,
                    Some(230400) => 10,
                    _ => return self.err("형식이 올바르지 않습니다."),
                };
                self.comma(c)?;
                if kw == "ETX" {
                    self.out.extend_from_slice(&[0xD6, baud]);
                    self.operand(c)?;
                } else {
                    let Some(Tok::Id(n)) = c.next() else {
                        return self.err("형식이 올바르지 않습니다.");
                    };
                    let v = self.var(&n)?;
                    self.comma(c)?;
                    let l = self.label_name(c)?;
                    self.out.extend_from_slice(&[0xD5, baud]);
                    self.out
                        .extend_from_slice(&[if v.int { 0x16 } else { 0x15 }, v.addr]);
                    self.jump_label(&l);
                }
            }
            "MUSIC" => {
                let Some(Tok::Str(s)) = c.next() else {
                    return self.err("형식이 맞지 않습니다.");
                };
                self.out.push(0xBA);
                let fix = self.placeholder();
                self.music(&s)?;
                self.patch_here(fix);
            }
            "IF" => {
                self.out.push(0xC0);
                let false_fix = self.placeholder();
                self.expr(c)?;
                if !c.eat_id("THEN") {
                    return self.err("[IF] 형식이 맞지 않습니다.");
                }
                let inline = !c.done();
                self.ifs.push(IfFrame {
                    false_fix: Some(false_fix),
                    end_fixes: vec![],
                    inline,
                });
                if inline {
                    self.statement(c)?;
                    if !c.done() {
                        return self.unsupported("IF su una riga con ELSE");
                    }
                    self.close_if();
                }
            }
            "ELSEIF" | "ELSE" => {
                let Some(top) = self.ifs.last() else {
                    return self.err("[ELSE] IF 없이 사용되었습니다.");
                };
                if top.inline {
                    return self.err("[ELSE] IF 없이 사용되었습니다.");
                }
                self.out.push(0xC4);
                let end = self.placeholder();
                let ff = self.ifs.last_mut().unwrap().false_fix.take();
                if let Some(ff) = ff {
                    self.patch_here(ff);
                }
                self.ifs.last_mut().unwrap().end_fixes.push(end);
                if kw == "ELSEIF" {
                    self.out.push(0xC0);
                    let f = self.placeholder();
                    self.expr(c)?;
                    if !c.eat_id("THEN") {
                        return self.err("[IF] 형식이 맞지 않습니다.");
                    }
                    self.ifs.last_mut().unwrap().false_fix = Some(f);
                }
            }
            "ENDIF" => {
                if self.ifs.last().is_none_or(|f| f.inline) {
                    return self.err("[ENDIF] IF 없이 사용되었습니다.");
                }
                self.close_if();
            }
            "REMARK" => {
                c.pos = c.toks.len();
            }
            _ => return self.unsupported(&kw),
        }
        Ok(())
    }

    fn print(&mut self, c: &mut Cur) -> Res<()> {
        self.out.push(0xE3);
        if c.eat_id("FORMAT") {
            if !c.eat_op("(") {
                return self.err("형식이 맞지 않습니다.");
            }
            let name = self.label_name(c)?;
            let v = self.var(&name)?;
            self.comma(c)?;
            let mode = self.label_name(c)?;
            let (tag, default) = match (mode.as_str(), v.int) {
                ("HEX", false) => (0x19, 2),
                ("HEX", true) => (0x19, 4),
                ("DEC", false) => (0x1A, 3),
                ("DEC", true) => (0x1A, 5),
                ("BIN", false) => (0x1B, 8),
                ("BIN", true) => (0x1B, 16),
                _ => return self.err("Formato PRINT non valido"),
            };
            let width = if c.eat_op(",") {
                match self.const_num(c)? {
                    Some(n @ 1..=255) => n as u8,
                    _ => return self.err("numero fuori intervallo"),
                }
            } else {
                default
            };
            if !c.eat_op(")") {
                return self.err("형식이 맞지 않습니다.");
            }
            self.out.push(tag);
            self.put_var(v);
            self.out.push(width);
            return Ok(());
        }
        self.out.push(0x18);
        loop {
            match c.peek() {
                Some(Tok::Str(s)) if s.is_ascii() => {
                    self.out.extend_from_slice(s.as_bytes());
                    c.pos += 1;
                }
                _ => match self.const_num(c)? {
                    Some(n @ 1..=255) => self.out.push(n as u8),
                    Some(_) => return self.err("numero fuori intervallo"),
                    None => return self.err("PRINT richiede caratteri o FORMAT"),
                },
            }
            if !c.eat_op(",") {
                break;
            }
        }
        self.out.push(0);
        Ok(())
    }

    // Coppie (durata, nota) terminate da 0. Durata = 96/n (default 4), ottava 3 = C 0x24.
    fn music(&mut self, s: &str) -> Res<()> {
        let bad = || CompileError {
            line: self.line,
            msg: "연주 문자열이 잘못되었습니다.".into(),
            unsupported: false,
        };
        let (mut octave, mut duration) = (3i32, 24f64);
        let mut sharp = 0i32;
        let b = s.to_ascii_uppercase().into_bytes();
        let mut i = 0;
        let mut out = Vec::new();
        let number = |i: &mut usize| -> Option<i32> {
            let st = *i;
            while *i < b.len() && b[*i].is_ascii_digit() {
                *i += 1;
            }
            std::str::from_utf8(&b[st..*i]).ok()?.parse().ok()
        };
        while i < b.len() {
            let ch = b[i];
            i += 1;
            match ch {
                b'O' => {
                    let digit = b
                        .get(i)
                        .filter(|d| (b'0'..=b'7').contains(d))
                        .ok_or_else(bad)?;
                    octave = i32::from(*digit - b'0');
                    i += 1;
                }
                b'L' | b'<' => octave -= 1,
                b'0'..=b'9' => {
                    i -= 1;
                    let n = number(&mut i)
                        .filter(|n| (0..=96).contains(n))
                        .ok_or_else(bad)?;
                    duration = 96.0 / f64::from(n.max(1));
                }
                b'H' | b'>' => octave += 1,
                b'M' => octave = 3,
                b'[' => duration /= 1.5,
                b']' => duration *= 1.5,
                b'#' | b'+' => sharp = 1,
                b'$' | b'-' => sharp = -1,
                ch if ch.is_ascii_whitespace() => {}
                b'A'..=b'G' | b'P' | b',' => {
                    let n = match ch {
                        b'C' => 0,
                        b'D' => 2,
                        b'E' => 4,
                        b'F' => 5,
                        b'G' => 7,
                        b'A' => 9,
                        b'B' => 11,
                        _ => -1,
                    };
                    let note = if n < 0 { 0x7F } else { octave * 12 + n + sharp };
                    if !duration.is_finite() || !(1.0..256.0).contains(&duration) {
                        return Err(bad());
                    }
                    out.push(duration as i32);
                    out.push(note);
                    sharp = 0;
                }
                _ => return Err(bad()),
            }
        }
        for v in out {
            if !(0..=255).contains(&v) {
                return Err(bad());
            }
            self.out.push(v as u8);
        }
        self.out.push(0);
        Ok(())
    }

    fn close_if(&mut self) {
        let f = self.ifs.pop().unwrap();
        if let Some(ff) = f.false_fix {
            self.patch_here(ff);
        }
        for e in f.end_fixes {
            self.patch_here(e);
        }
    }
}

/// Compila `src`. `name` è il nome (max 8 caratteri) scritto nell'header;
/// `stamp` = [giorno, mese, anno%100, ore, minuti, secondi].
pub fn compile(src: &str, name: &str, stamp: [u8; 6]) -> Res<Vec<u8>> {
    match compile_inner(src, name, stamp) {
        // L'originale con più di 192 byte di variabili non segnala errore: produce un programma vuoto
        Err(e) if e.msg == "변수 영역 초과" => {
            let mut b = compile_inner("END\r\n", name, stamp)?;
            b.truncate(b.len() - 3);
            b[15] -= 3;
            Ok(b)
        }
        r => r,
    }
}

fn compile_inner(src: &str, name: &str, stamp: [u8; 6]) -> Res<Vec<u8>> {
    let mut c = Compiler {
        out: Vec::new(),
        line: 0,
        vars: HashMap::new(),
        consts: HashMap::new(),
        next_var: 0x40,
        labels: HashMap::new(),
        fixups: Vec::new(),
        ifs: Vec::new(),
        fors: Vec::new(),
    };
    for (line, toks) in expand_poses(src)? {
        c.line = line;
        if toks.len() == 2 && matches!(toks[1], Tok::Op(ref o) if o == ":") {
            if let Tok::Id(l) = &toks[0] {
                if l == "AUTO" {
                    return c.err("AUTO e' riservato al template firmware");
                }
                let h = c.here();
                if c.labels.insert(l.clone(), h).is_some() {
                    return c.err("레이블이 중복되었습니다.");
                }
                continue;
            }
        }
        let mut cur = Cur { toks, pos: 0 };
        c.statement(&mut cur)?;
        // Riservare header e salto finale prima che un indirizzo possa andare in wrap.
        if BODY_BASE + c.out.len() + 3 > usize::from(u16::MAX) + 1 {
            return c.err("Programma troppo grande per gli indirizzi a 16 bit");
        }
        if !cur.done() {
            return c.err("형식이 맞지 않습니다.");
        }
    }
    if !c.ifs.is_empty() {
        return c.err("[IF] ENDIF가 없습니다.");
    }
    if !c.fors.is_empty() {
        return c.err("[FOR] NEXT가 없습니다.");
    }
    let h = c.here();
    c.out.push(0xC4);
    c.put_addr(h);
    for (at, label, line) in std::mem::take(&mut c.fixups) {
        let Some(a) = c.labels.get(&label) else {
            return Err(CompileError {
                line,
                msg: format!("레이블 {label} 없음"),
                unsupported: false,
            });
        };
        let b = a.to_le_bytes();
        c.out[at] = b[0];
        c.out[at + 1] = b[1];
    }
    let mut bin = vec![0u8];
    let mut n: Vec<u8> = name.bytes().take(8).collect();
    n.resize(8, b' ');
    bin.extend_from_slice(&n);
    bin.extend_from_slice(&stamp);
    let total = BODY_BASE + c.out.len();
    bin.extend_from_slice(&((total - 1) as u16).to_le_bytes());
    bin.extend_from_slice(&c.out);
    Ok(bin)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn samples() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../re/samples")
    }

    // Confronta con l'output del compilatore originale, ignorando il timestamp (byte 9..15).
    #[test]
    fn matches_original_compiler() {
        #[derive(serde::Deserialize)]
        struct Expected {
            name: String,
            error: usize,
            line: usize,
        }
        let inventory: Vec<Expected> = serde_json::from_str(
            &std::fs::read_to_string(samples().join("expectations.json")).unwrap(),
        )
        .unwrap();
        let expected_names: std::collections::HashSet<_> =
            inventory.iter().map(|e| e.name.clone()).collect();
        let actual_names: std::collections::HashSet<_> = std::fs::read_dir(samples())
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("bas"))
            .map(|p| p.file_stem().unwrap().to_string_lossy().to_string())
            .collect();
        assert_eq!(
            expected_names, actual_names,
            "Inventario dei campioni modificato"
        );
        assert_eq!(expected_names.len(), inventory.len(), "Campioni duplicati");
        for e in &inventory {
            let p = samples().join(format!("{}.bas", e.name));
            assert_eq!(
                p.with_extension("hex").exists(),
                e.error == 0,
                "{}: esito originale incoerente",
                e.name
            );
            if e.error != 0 {
                let got = compile(&std::fs::read_to_string(p).unwrap(), "        ", [0; 6])
                    .expect_err(&format!("{} accettato ma rifiutato dall'originale", e.name));
                assert_eq!(got.line, e.line, "{}: riga errore diversa", e.name);
            }
        }
        let mut checked = 0;
        let mut skipped = Vec::new();
        let mut bad = Vec::new();
        let mut entries: Vec<_> = std::fs::read_dir(samples()).unwrap().flatten().collect();
        entries.sort_by_key(|e| e.path());
        for e in entries {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) != Some("hex") {
                continue;
            }
            let name = p.file_stem().unwrap().to_string_lossy().to_string();
            let expected: Vec<u8> = std::fs::read_to_string(&p)
                .unwrap()
                .split_whitespace()
                .map(|h| u8::from_str_radix(h, 16).unwrap())
                .collect();
            let src = std::fs::read_to_string(p.with_extension("bas")).unwrap();
            match compile(&src, "        ", [0; 6]) {
                Ok(mut got) => {
                    let mut exp = expected.clone();
                    // Il nome nei campioni è sempre 8 spazi; il timestamp varia.
                    got[9..15].copy_from_slice(&[0; 6]);
                    exp[9..15].copy_from_slice(&[0; 6]);
                    if got != exp {
                        bad.push(format!("{name}\n  got {got:02X?}\n  exp {exp:02X?}"));
                    }
                    checked += 1;
                }
                Err(e) if e.unsupported => skipped.push(name),
                Err(e) => bad.push(format!("{name}: errore riga {}: {}", e.line, e.msg)),
            }
        }
        eprintln!("verificati {checked}; non supportati: {skipped:?}");
        // Sorgenti rifiutati dall'originale (nessun .hex) devono essere rifiutati anche da noi
        let mut accepted = Vec::new();
        for e in std::fs::read_dir(samples()).unwrap().flatten() {
            let p = e.path();
            if p.extension().and_then(|x| x.to_str()) == Some("bas")
                && !p.with_extension("hex").exists()
            {
                let src = std::fs::read_to_string(&p).unwrap();
                if compile(&src, "        ", [0; 6]).is_ok() {
                    accepted.push(p.file_stem().unwrap().to_string_lossy().to_string());
                }
            }
        }
        eprintln!("accettati ma rifiutati dall'originale: {accepted:?}");
        assert!(bad.is_empty(), "{} diversi:\n{}", bad.len(), bad.join("\n"));
        assert!(
            accepted.is_empty(),
            "accettati ma rifiutati dall'originale: {accepted:?}"
        );
        assert!(skipped.is_empty(), "campioni non supportati: {skipped:?}");
        assert_eq!(checked, inventory.iter().filter(|e| e.error == 0).count());
    }

    #[test]
    fn header_layout() {
        let b = compile("END\r\n", "TEST", [26, 10, 26, 12, 36, 8]).unwrap();
        assert_eq!(
            b,
            [
                0, b'T', b'E', b'S', b'T', 32, 32, 32, 32, 26, 10, 26, 12, 36, 8, 0x16, 0, 0xC4,
                0x10, 0, 0xC4, 0x13, 0
            ]
        );
    }

    #[test]
    fn undefined_label_is_error() {
        assert!(compile("GOTO X\r\n", "", [0; 6]).is_err());
    }

    #[test]
    fn oversized_programs_never_wrap_header_lengths_or_jump_addresses() {
        let source = format!("PRINT \"{}\"\nEND", "A".repeat(usize::from(u16::MAX) - 25));
        let object = compile(&source, "", [0; 6]).unwrap();
        assert_eq!(object.len(), usize::from(u16::MAX) + 1);
        assert_eq!(u16::from_le_bytes([object[15], object[16]]), u16::MAX);
        for source in [
            source.replace("PRINT \"", "PRINT \"A"),
            format!("{}END", "DELAY 100\n".repeat(17000)),
        ] {
            let e = compile(&source, "", [0; 6]).unwrap_err();
            assert!(e.msg.contains("16 bit"));
            assert!(e.line > 0);
        }
    }

    #[test]
    fn movepos_lowers_to_original_move_bytecode() {
        for (fixture, src) in [
            (
                "manual_pose_expanded",
                "MOVEPOS P1\nEND\nP1: POS G6A,100,,120\nPOS G6B,90,100,110\n",
            ),
            (
                "manual_pose_if_expanded",
                "DIM A AS BYTE\nA=1\nIF A=1 THEN MOVEPOS P1\nEND\nP1: POS G6A,100,,120\nPOS G6B,90,100,110\n",
            ),
        ] {
            let canonical =
                std::fs::read_to_string(samples().join(format!("{fixture}.bas"))).unwrap();
            assert_eq!(
                compile(src, "POSE", [0; 6]).unwrap(),
                compile(&canonical, "POSE", [0; 6]).unwrap()
            );
        }
        for src in [
            "MOVEPOS MISSING\nEND",
            "MOVEPOS P1\nEND\nP1:\nDELAY 10",
            "MOVEPOS P1\nEND\nP1: POS G6A,255",
            "DIM POS AS BYTE\nDIM MOVE AS BYTE\nMOVEPOS P1\nEND\nP1: POS=1",
        ] {
            assert!(compile(src, "", [0; 6]).is_err());
        }
    }

    #[test]
    fn byte_bit_access_cannot_overwrite_the_next_variable() {
        for src in [
            "DIM A AS BYTE\nA.8 = 1",
            "DIM A AS BYTE\nDIM B AS BYTE\nB = A.8",
        ] {
            let e = compile(src, "", [0; 6]).unwrap_err();
            assert!(e.msg.contains("BYTE"));
            assert_eq!(e.line, src.lines().count());
        }
    }
}
