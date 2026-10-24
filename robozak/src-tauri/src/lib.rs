mod basic;

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
    let num = |m: &HashMap<String, String>, k: &str| {
        m.get(k).and_then(|v| v.parse().ok()).unwrap_or(0)
    };
    let mut robot = Robot { image: String::new(), zero_image: String::new(), motors: Vec::new() };
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

#[derive(Serialize)]
struct Simulation {
    robot: basic::SimRobot,
    vars: std::collections::HashMap<String, i32>,
    stop: basic::Stop,
}

#[tauri::command]
fn check_program(src: &str) -> Result<(), basic::ParseError> {
    basic::parse(src, &parse_keywords(KEYWORDS)).map(|_| ())
}

#[tauri::command]
fn simulate(src: &str, max_steps: usize) -> Result<Simulation, String> {
    let prog = basic::parse(src, &parse_keywords(KEYWORDS))
        .map_err(|e| format!("Riga {}: {}", e.line, e.msg))?;
    let mut robot = basic::SimRobot::default();
    let (stop, vars) = {
        let mut i = basic::Interp::new(&prog, &mut robot);
        let stop = i.run(max_steps).map_err(|e| format!("Riga {}: {}", e.line, e.msg))?;
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

#[tauri::command]
fn serial_open(port: State<Port>, name: &str) -> Result<(), String> {
    let p = serialport::new(name, 115_200)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .timeout(Duration::from_millis(500))
        .open()
        .map_err(|e| e.to_string())?;
    *port.lock().unwrap() = Some(p);
    Ok(())
}

#[tauri::command]
fn serial_close(port: State<Port>) {
    *port.lock().unwrap() = None;
}

#[tauri::command]
fn serial_write(port: State<Port>, data: Vec<u8>) -> Result<(), String> {
    let mut g = port.lock().unwrap();
    let p = g.as_mut().ok_or("Porta seriale non aperta")?;
    p.write_all(&data).map_err(|e| e.to_string())
}

#[tauri::command]
fn serial_read(port: State<Port>, max: usize) -> Result<Vec<u8>, String> {
    let mut g = port.lock().unwrap();
    let p = g.as_mut().ok_or("Porta seriale non aperta")?;
    let mut buf = vec![0u8; max];
    match p.read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            Ok(buf)
        }
        Err(e) if e.kind() == std::io::ErrorKind::TimedOut => Ok(Vec::new()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Port::default())
        .invoke_handler(tauri::generate_handler![keywords, robot, serial_ports, check_program, simulate, serial_open, serial_close, serial_write, serial_read])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
