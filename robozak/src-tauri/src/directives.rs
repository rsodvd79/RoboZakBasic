use crate::basic::{Expr, Op, Program, const_eval, group_motors};
use crate::compiler::{CompileError, Tok, lex};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ServoLimit {
    pub min: u8,
    pub max: u8,
    pub declared: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Settings {
    pub limits: Vec<ServoLimit>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            limits: vec![
                ServoLimit {
                    min: 10,
                    max: 190,
                    declared: false
                };
                24
            ],
        }
    }
}

pub fn parse(src: &str) -> Result<Settings, CompileError> {
    let mut settings = Settings::default();
    for (i, raw) in src.lines().enumerate() {
        let Some(comment) = raw.trim_start().strip_prefix('\'') else {
            continue;
        };
        let Some(directive) = comment.trim_start().strip_prefix('$') else {
            continue;
        };
        let directive = directive.trim();
        let (kind, rest) = directive
            .split_once(char::is_whitespace)
            .unwrap_or((directive, ""));
        let error = |msg: &str, unsupported| CompileError {
            line: i + 1,
            msg: msg.into(),
            unsupported,
        };
        match kind.to_ascii_uppercase().as_str() {
            "DEVICE" => {
                let device = rest
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_ascii_uppercase()
                    .replace('-', "");
                if !matches!(device.as_str(), "MRC3024" | "MRC3000") {
                    return Err(error("$DEVICE: supportato solo MR-C3024 / MRC3000", true));
                }
            }
            "LIMIT" => {
                let toks = lex(rest);
                let Some(
                    [
                        Tok::Num(motor),
                        Tok::Op(comma1),
                        Tok::Num(min),
                        Tok::Op(comma2),
                        Tok::Num(max),
                    ],
                ) = toks.get(..5)
                else {
                    return Err(error(
                        "$LIMIT richiede motore, minimo, massimo numerici",
                        false,
                    ));
                };
                if comma1 != ","
                    || comma2 != ","
                    || (toks.len() > 5 && !matches!(toks[5], Tok::Id(_)))
                {
                    return Err(error("Sintassi $LIMIT non valida", false));
                }
                if !(0..24).contains(motor) {
                    return Err(error(
                        "$LIMIT: motore MR-C3024 fuori intervallo (0-23)",
                        false,
                    ));
                }
                if !(10..=190).contains(min) || !(10..=190).contains(max) || min > max {
                    return Err(error("$LIMIT: usare 10 <= minimo <= massimo <= 190", false));
                }
                settings.limits[*motor as usize] = ServoLimit {
                    min: *min as u8,
                    max: *max as u8,
                    declared: true,
                };
            }
            _ => return Err(error("Direttiva non supportata per MR-C3024", true)),
        }
    }
    Ok(settings)
}

impl Settings {
    pub fn has_limits(&self) -> bool {
        self.limits.iter().any(|limit| limit.declared)
    }

    pub fn check_position(&self, motor: usize, position: i32) -> Result<(), String> {
        if let Some(limit) = self.limits.get(motor).filter(|limit| limit.declared)
            && (position < i32::from(limit.min) || position > i32::from(limit.max))
        {
            return Err(format!(
                "Motore {motor}: posizione {position} fuori dal limite {}-{}",
                limit.min, limit.max
            ));
        }
        Ok(())
    }

    // Le direttive non sono bytecode firmware: non scaricare movimenti non verificabili.
    pub fn validate(&self, program: &Program) -> Result<(), CompileError> {
        if !self.has_limits() {
            return Ok(());
        }
        for (i, op) in program.ops.iter().enumerate() {
            let Op::Cmd(name, args) = op else { continue };
            let error = |msg: String, unsupported| CompileError {
                line: program.lines[i],
                msg,
                unsupported,
            };
            if name == "SERVO" {
                let motor = args
                    .first()
                    .and_then(Option::as_ref)
                    .and_then(|e| const_eval(e, &program.consts));
                let Some(motor) = motor else {
                    return Err(error(
                        "$LIMIT: motore dinamico non verificabile per il download".into(),
                        true,
                    ));
                };
                if self
                    .limits
                    .get(motor as usize)
                    .is_some_and(|limit| limit.declared)
                {
                    let position = args
                        .get(1)
                        .and_then(Option::as_ref)
                        .and_then(|e| const_eval(e, &program.consts));
                    let Some(position) = position else {
                        return Err(error(
                            "$LIMIT: posizione dinamica non verificabile per il download".into(),
                            true,
                        ));
                    };
                    self.check_position(motor as usize, position)
                        .map_err(|msg| error(msg, false))?;
                }
            } else if matches!(name.as_str(), "MOVE" | "MOVE24" | "POS" | "INIT") {
                let (motors, values) = if name == "MOVE24" {
                    ((0..24).collect::<Vec<_>>(), args.as_slice())
                } else if let Some(Some(Expr::Var(group))) = args.first() {
                    (group_motors(group), &args[1..])
                } else {
                    return Err(error("Gruppo motori non verificabile".into(), true));
                };
                for (motor, value) in motors.into_iter().zip(values) {
                    if let Some(position) = value
                        .as_ref()
                        .and_then(|e| const_eval(e, &program.consts))
                        .filter(|p| *p != 0)
                    {
                        self.check_position(motor, position)
                            .map_err(|msg| error(msg, false))?;
                    }
                }
            } else if matches!(name.as_str(), "ACTION" | "FUNC" | "POKE" | "ROMPOKE")
                || (name == "AIMOTOR" && args.first() == Some(&Some(Expr::Var("INIT".into()))))
            {
                return Err(error(
                    format!(
                        "$LIMIT: {name} puo' modificare posizioni non verificabili; download non consentito"
                    ),
                    true,
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_device_and_global_limits() {
        let s = parse("' $device MR-C3024\n'$LIMIT 0,50,100\n'$LIMIT 0,60,90\n'$LIMIT 23,&H0A,190 ' fine\nEND").unwrap();
        assert!(s.check_position(0, 60).is_ok());
        assert!(s.check_position(0, 100).is_err());
        assert!(s.check_position(1, 150).is_ok());
        assert!(parse("' normale commento\n'$DEVICE MRC3000\nEND").is_ok());
    }

    #[test]
    fn invalid_directives_report_their_line() {
        for directive in [
            "'$DEVICE MRC2000",
            "'$LIMIT 24,50,100",
            "'$LIMIT 0,9,100",
            "'$LIMIT 0,100,50",
            "'$LIMIT 0,50,191",
            "'$LIMIT 0,50",
            "'$LIMIT 0,50,100,150",
            "'$LIMIT 0,X,100",
            "'$UNKNOWN 1",
        ] {
            assert_eq!(
                parse(&format!("' commento\n{directive}\nEND"))
                    .unwrap_err()
                    .line,
                2
            );
        }
    }

    #[test]
    fn declared_limits_guard_direct_and_indirect_movements() {
        let settings = parse("'$LIMIT 0,50,100").unwrap();
        let kw = crate::parse_keywords(include_str!("../resources/mrc3000e.cmd"));
        for body in [
            "SERVO 0,150",
            "MOVE G6A,150",
            "MOVE24 150",
            "INIT G6A,150",
            "MOVEPOS P1\nEND\nP1: POS G6A,150",
            "DIM A AS BYTE\nSERVO 0,A",
            "DIM A AS BYTE\nSERVO A,90",
            "ACTION 1",
            "FUNC 1",
            "AIMOTOR INIT",
            "ROMPOKE 0,1",
            "POKE 64,1",
            "CONST P = 150\nSERVO 0,P\nCONST P = 50",
            "CONST M = 0\nSERVO M,150\nCONST M = 1",
            "CONST INIT = 50\nAIMOTOR INIT",
        ] {
            let p = crate::basic::parse(body, &kw).unwrap();
            assert!(settings.validate(&p).is_err(), "{body}");
        }
        for body in [
            "SERVO 0,50",
            "SERVO 0,100",
            "CONST P = 90\nSERVO 0,P",
            "CONST P = 90\nSERVO 0,P\nCONST P = 150",
            "MOVE G6A,,150",
            "DIM A AS BYTE\nSERVO 1,A",
            "ZERO G6A,110",
        ] {
            let p = crate::basic::parse(body, &kw).unwrap();
            settings.validate(&p).unwrap();
        }
    }
}
