mod basic;
pub mod compiler;
mod directives;
pub mod download;

use serde::Serialize;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::Mutex;
use std::time::Duration;
use tauri::State;

const KEYWORDS: &str = include_str!("../resources/mrc3000e.cmd");
const ROBONOVA: &str = include_str!("../resources/robonova.dat");
const ROKI: &str = include_str!("../resources/roki.dat");

#[derive(Serialize, Debug, PartialEq)]
pub struct Motor {
    pub no: i32,
    pub group: i32,
    pub left: i32,
    pub top: i32,
    pub used: bool,
    pub opposite: i32,
    pub zero_left: i32,
    pub zero_top: i32,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Robot {
    pub image: String,
    pub zero_image: String,
    pub motors: Vec<Motor>,
}

pub fn parse_keywords(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.split(',').next().unwrap_or("").trim())
        .filter(|k| !k.is_empty())
        .map(String::from)
        .collect()
}

fn parse_ini(text: &str) -> Vec<(String, HashMap<String, String>)> {
    let mut sections: Vec<(String, HashMap<String, String>)> = Vec::new();
    for line in text.lines().map(str::trim) {
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            sections.push((name.to_string(), HashMap::new()));
        } else if let (Some((k, v)), Some(last)) = (line.split_once('='), sections.last_mut()) {
            last.1.insert(k.trim().to_string(), v.trim().to_string());
        }
    }
    sections
}

pub fn parse_robot(text: &str) -> Robot {
    let sections = parse_ini(text);
    let num =
        |m: &HashMap<String, String>, k: &str| m.get(k).and_then(|v| v.parse().ok()).unwrap_or(0);
    let mut robot = Robot {
        image: String::new(),
        zero_image: String::new(),
        motors: Vec::new(),
    };
    for (name, props) in &sections {
        if name == "PICTURE" {
            robot.image = props.get("Robot").cloned().unwrap_or_default();
            robot.zero_image = props.get("Zero").cloned().unwrap_or_default();
        } else if name.starts_with("MOTOR") && name.ends_with("_PROPERTY") {
            robot.motors.push(Motor {
                no: num(props, "MotorNo"),
                group: num(props, "MotorGroup"),
                left: num(props, "MotorLeft"),
                top: num(props, "MotorTop"),
                used: num(props, "MotorUse") != 0,
                opposite: num(props, "MotorOpp"),
                zero_left: num(props, "ZeroLeft"),
                zero_top: num(props, "ZeroTop"),
            });
        }
    }
    robot
}

#[tauri::command]
fn keywords() -> Vec<String> {
    parse_keywords(KEYWORDS)
}

#[tauri::command]
fn robot(name: &str) -> Result<Robot, String> {
    match name {
        "robonova" => Ok(parse_robot(ROBONOVA)),
        "roki" => Ok(parse_robot(ROKI)),
        _ => Err(format!("unknown robot: {name}")),
    }
}

#[tauri::command]
fn read_source(path: &str) -> Result<String, String> {
    let bytes =
        std::fs::read(path).map_err(|e| format!("Impossibile caricare il sorgente: {e}"))?;
    let text = if let Some(bytes) = bytes.strip_prefix(&[0xFF, 0xFE]) {
        encoding_rs::UTF_16LE.decode(bytes).0.into_owned()
    } else if let Some(bytes) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        encoding_rs::UTF_16BE.decode(bytes).0.into_owned()
    } else {
        let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(&bytes);
        match std::str::from_utf8(bytes) {
            Ok(text) => text.to_owned(),
            Err(_) => encoding_rs::WINDOWS_1252.decode(bytes).0.into_owned(),
        }
    };
    if text.contains('\0') {
        return Err("Il file contiene dati binari, non un sorgente BASIC".into());
    }
    Ok(text)
}

#[tauri::command]
fn write_source(path: &str, src: &str) -> Result<(), String> {
    std::fs::write(path, src.as_bytes())
        .map_err(|e| format!("Impossibile salvare il sorgente: {e}"))
}

#[derive(Serialize)]
struct Simulation {
    robot: basic::SimRobot,
    vars: std::collections::HashMap<String, i32>,
    stop: basic::Stop,
}

#[derive(Debug, Serialize)]
struct Diagnostic {
    line: usize,
    msg: String,
    unsupported: bool,
}

impl From<compiler::CompileError> for Diagnostic {
    fn from(e: compiler::CompileError) -> Self {
        let msg = if e.unsupported {
            format!("Funzionalita non supportata per MR-C3024: {}", e.msg)
        } else if e.msg.contains("선언되어") {
            "Variabile o costante non dichiarata".into()
        } else if e.msg.contains("레이블") {
            "Etichetta mancante o duplicata".into()
        } else if e.msg.contains("범위") || e.msg == "numero fuori intervallo" {
            "Valore fuori intervallo".into()
        } else if e.msg.contains("연주") {
            "Stringa MUSIC non valida".into()
        } else if e.msg.contains("ENDIF") {
            "Blocco IF/ENDIF non valido".into()
        } else if e.msg.contains("NEXT") || e.msg.contains("FOR") {
            "Ciclo FOR/NEXT non valido (STEP non supportato)".into()
        } else if e.msg.is_ascii() {
            e.msg.clone()
        } else {
            "Sintassi non valida per MR-C3024".into()
        };
        Self {
            line: e.line,
            msg,
            unsupported: e.unsupported,
        }
    }
}

#[tauri::command]
fn check_program(src: &str) -> Result<(), Diagnostic> {
    compile_program(src, "", [0; 6]).map(|_| ())
}

fn compile_program(src: &str, name: &str, stamp: [u8; 6]) -> Result<Vec<u8>, Diagnostic> {
    let settings = program_settings(src)?;
    let object = compiler::compile(src, name, stamp).map_err(Diagnostic::from)?;
    if settings.has_limits() {
        let program = basic::parse(src, &parse_keywords(KEYWORDS)).map_err(|e| Diagnostic {
            line: e.line,
            msg: e.msg,
            unsupported: true,
        })?;
        settings.validate(&program).map_err(Diagnostic::from)?;
    }
    Ok(object)
}

#[tauri::command]
fn program_settings(src: &str) -> Result<directives::Settings, Diagnostic> {
    directives::parse(src).map_err(Into::into)
}

#[tauri::command]
fn simulate(src: &str, max_steps: usize) -> Result<Simulation, Diagnostic> {
    check_program(src)?;
    let prog = basic::parse(src, &parse_keywords(KEYWORDS)).map_err(|e| Diagnostic {
        line: e.line,
        msg: "Sintassi non gestita dal simulatore".into(),
        unsupported: true,
    })?;
    let mut robot = basic::SimRobot {
        settings: program_settings(src)?,
        ..basic::SimRobot::default()
    };
    let (stop, vars) = {
        let mut i = basic::Interp::new(&prog, &mut robot);
        let stop = i.run(max_steps).map_err(|e| Diagnostic {
            line: e.line,
            unsupported: e.msg.contains("non simulata"),
            msg: if e.msg.is_ascii()
                && (e.msg.contains("simulat")
                    || e.msg.contains("Motore")
                    || e.msg.starts_with("Assegnazione di bit"))
            {
                e.msg
            } else {
                "Errore durante la simulazione".into()
            },
        })?;
        (stop, i.vars)
    };
    Ok(Simulation { robot, vars, stop })
}

#[tauri::command]
fn serial_ports() -> Result<Vec<String>, String> {
    serialport::available_ports()
        .map(|p| p.into_iter().map(|i| i.port_name).collect())
        .map_err(|e| e.to_string())
}

type Port = Mutex<Option<Box<dyn serialport::SerialPort>>>;

fn with_connection<P, T>(
    port: &Mutex<Option<P>>,
    operation: impl FnOnce(&mut P) -> Result<T, String>,
) -> Result<T, String> {
    let mut g = port.lock().map_err(|_| "Stato seriale non disponibile")?;
    let p = g.as_mut().ok_or("Porta seriale non aperta")?;
    let result = operation(p);
    if result.is_err() {
        *g = None;
    }
    result
}

#[tauri::command]
fn serial_open(port: State<Port>, name: &str) -> Result<(), String> {
    let mut g = port.lock().map_err(|_| "Stato seriale non disponibile")?;
    if g.is_some() {
        return Err("Porta seriale gia aperta".into());
    }
    let mut p = serialport::new(name, 9_600)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_millis(500))
        .open()
        .map_err(|e| e.to_string())?;
    download::supported_controller(download::controller_info(&mut p)?)?;
    *g = Some(p);
    Ok(())
}

#[tauri::command]
fn serial_close(port: State<Port>) {
    *port.lock().unwrap() = None;
}

#[tauri::command]
fn serial_is_open(port: State<Port>) -> bool {
    port.lock().is_ok_and(|g| g.is_some())
}

#[tauri::command]
fn serial_write(port: State<Port>, data: Vec<u8>) -> Result<(), String> {
    with_connection(&port, |p| p.write_all(&data).map_err(|e| e.to_string()))
}

#[tauri::command]
fn servo_set(port: State<Port>, motor: u8, pos: u8, src: Option<String>) -> Result<(), String> {
    validate_live_servo(src.as_deref().unwrap_or(""), motor, pos)?;
    with_connection(&port, |p| download::servo(p, motor, pos))
}

fn validate_live_servo(src: &str, motor: u8, pos: u8) -> Result<(), String> {
    let settings = directives::parse(src).map_err(|e| format!("Riga {}: {}", e.line, e.msg))?;
    settings.check_position(motor as usize, i32::from(pos))
}

#[tauri::command]
fn serial_read(port: State<Port>, max: usize) -> Result<Vec<u8>, String> {
    with_connection(&port, |p| {
        let mut buf = vec![0u8; max];
        match p.read(&mut buf) {
            Ok(n) => {
                buf.truncate(n);
                Ok(buf)
            }
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(Vec::new()),
            Err(e) => Err(e.to_string()),
        }
    })
}

#[tauri::command]
fn download_program(
    port: State<Port>,
    src: &str,
    name: &str,
    stamp: [u8; 6],
) -> Result<usize, Diagnostic> {
    let obj = compile_program(src, name, stamp)?;
    let serial_error = |msg| Diagnostic {
        line: 0,
        msg,
        unsupported: false,
    };
    let mut g = port
        .lock()
        .map_err(|_| serial_error("Stato seriale non disponibile".into()))?;
    // Il baud immediato dopo il trasferimento non e' misurato: chiudere anche sul successo.
    let mut p = g
        .take()
        .ok_or_else(|| serial_error("Porta seriale non aperta".into()))?;
    download::download(
        &mut p,
        &obj,
        name,
        &mut |p| {
            p.set_baud_rate(download::FAST_BAUD)
                .map_err(|e| e.to_string())
        },
        &mut |_, _| {},
    )
    .map_err(serial_error)?;
    Ok(obj.len())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Port::default())
        .invoke_handler(tauri::generate_handler![
            keywords,
            robot,
            read_source,
            write_source,
            serial_ports,
            check_program,
            program_settings,
            simulate,
            serial_open,
            serial_close,
            serial_is_open,
            serial_write,
            serial_read,
            download_program,
            servo_set
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_files_round_trip_and_read_legacy_text() {
        let path = std::env::temp_dir().join(format!(
            "robozak-source-{}-{}.bas",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let name = path.to_str().unwrap();
        let src = "' caff\u{e8}\r\nPRINT \"OK\"\r\nEND\r\n";
        write_source(name, src).unwrap();
        assert_eq!(read_source(name).unwrap(), src);
        std::fs::write(&path, b"\xEF\xBB\xBFEND\r\n").unwrap();
        assert_eq!(read_source(name).unwrap(), "END\r\n");
        std::fs::write(&path, b"' caff\xE8\r\nEND").unwrap();
        assert_eq!(read_source(name).unwrap(), "' caff\u{e8}\r\nEND");
        std::fs::write(&path, [0xFF, 0xFE, b'E', 0, b'N', 0, b'D', 0]).unwrap();
        assert_eq!(read_source(name).unwrap(), "END");
        std::fs::write(&path, b"binary\0file").unwrap();
        assert!(read_source(name).is_err());
        std::fs::remove_file(&path).unwrap();
        assert!(read_source(name).is_err());
        assert!(write_source(name, "END").is_ok());
        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn keywords_parsed() {
        let k = parse_keywords(KEYWORDS);
        assert!(k.len() > 100);
        assert_eq!(k[0], "DIM");
        assert!(k.contains(&"GYROSET".to_string()));
    }

    #[test]
    fn robonova_has_24_motors() {
        let r = parse_robot(ROBONOVA);
        assert_eq!(r.motors.len(), 24);
        assert_eq!(r.image, "robonova.bmp");
        assert_eq!(r.motors[0].zero_left, 300);
        assert_eq!(r.motors[1].left, 163);
    }

    #[test]
    fn validation_and_simulation_share_compiler_errors() {
        for src in [
            "DIM A AS BYTE\nA = NOT A",
            "DIM A AS BYTE\nA = (1 + 2)",
            "NO_SUCH_COMMAND",
        ] {
            let expected = compiler::compile(src, "", [0; 6]).unwrap_err();
            let check = check_program(src).unwrap_err();
            let sim = simulate(src, 10).err().unwrap();
            assert_eq!(check.line, expected.line);
            assert_eq!(sim.line, check.line);
            assert_eq!(sim.msg, check.msg);
            assert!(check.msg.is_ascii());
        }
        let sim = simulate("DIM A AS BYTE\nA = 1 + 2 * 3\nEND", 10).unwrap();
        assert_eq!(sim.vars["A"], 9);
    }

    #[test]
    fn serial_failure_invalidates_session() {
        let port = Mutex::new(Some(std::io::Cursor::new(vec![])));
        with_connection(&port, |p| p.write_all(&[1]).map_err(|e| e.to_string())).unwrap();
        assert!(port.lock().unwrap().is_some());
        assert!(with_connection::<_, ()>(&port, |_| Err("Timeout".into())).is_err());
        assert!(port.lock().unwrap().is_none());
        assert!(with_connection(&port, |_| Ok(())).is_err());
    }

    #[test]
    fn directive_guards_are_shared_by_check_simulation_and_download_compilation() {
        let src = "'$LIMIT 0,50,100\nSERVO 0,150\nEND";
        assert!(compiler::compile(src, "", [0; 6]).is_ok());
        let check = check_program(src).unwrap_err();
        let sim = simulate(src, 10).err().unwrap();
        let download = compile_program(src, "TEST", [0; 6]).unwrap_err();
        assert_eq!(check.line, 2);
        assert_eq!(check.msg, sim.msg);
        assert_eq!(check.msg, download.msg);
        assert!(check.msg.contains("50-100"));
        assert!(validate_live_servo(src, 0, 150).is_err());
        assert!(validate_live_servo(src, 0, 50).is_ok());
        let e = check_program("'$DEVICE MRC2000\nEND").unwrap_err();
        assert_eq!(e.line, 1);
        assert!(e.unsupported);
        assert!(validate_live_servo("'$DEVICE MRC2000", 0, 100).is_err());
    }
}
