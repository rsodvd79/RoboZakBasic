//! Download del programma compilato sul controller (protocollo ricavato da roboBASIC v2.72,
//! vedi re/NOTES.md). Il controller fa l'eco di ogni byte dei comandi; `3F` e i blocchi dati no.

use std::io::{Read, Write};

const BLOCK: usize = 128;
pub const MAX_PROGRAM_BYTES: usize = 32 * 1024;

fn send_echo<P: Read + Write>(p: &mut P, bytes: &[u8]) -> Result<(), String> {
    for &b in bytes {
        p.write_all(&[b]).map_err(|e| e.to_string())?;
        let r = read_byte(p)?;
        if r != b {
            return Err(format!("Eco errato: inviato {b:02X}, ricevuto {r:02X}"));
        }
    }
    Ok(())
}

fn read_byte<P: Read>(p: &mut P) -> Result<u8, String> {
    let mut b = [0u8; 1];
    match p.read(&mut b) {
        Ok(1) => Ok(b[0]),
        Ok(_) => Err("Timeout: il controller non risponde".into()),
        Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
            Err("Timeout: il controller non risponde".into())
        }
        Err(e) => Err(e.to_string()),
    }
}

/// Buffer inviato al controller: `FF`, nome (7 caratteri), data (6), lunghezza (2), codice.
/// Il compilatore produce `00`, nome (8), ...: il primo byte diventa `FF` e l'ottavo carattere del nome cade.
pub fn image(object: &[u8]) -> Result<Vec<u8>, String> {
    validate_object(object)?;
    let mut v = vec![0xFF];
    v.extend_from_slice(&object[1..8]);
    v.extend_from_slice(&object[9..]);
    Ok(v)
}

pub fn validate_object(object: &[u8]) -> Result<(), String> {
    if object.len() < 17 {
        return Err("Oggetto compilato troppo corto".into());
    }
    if object.len() - 1 > MAX_PROGRAM_BYTES {
        return Err(
            "Programma troppo grande per MR-C3024: massimo 32768 byte (32 KB), header incluso"
                .into(),
        );
    }
    if usize::from(u16::from_le_bytes([object[15], object[16]])) != object.len() - 1 {
        return Err("Lunghezza dell'oggetto compilato incoerente".into());
    }
    Ok(())
}

/// Blocchi da 128 byte (zero-padded) seguiti dal checksum (somma modulo 256).
pub fn blocks(image: &[u8]) -> Vec<Vec<u8>> {
    image
        .chunks(BLOCK)
        .map(|c| {
            let mut b = c.to_vec();
            b.resize(BLOCK, 0);
            let sum = b.iter().fold(0u8, |a, &x| a.wrapping_add(x));
            b.push(sum);
            b
        })
        .collect()
}

/// Informazioni del controller: (tipo, versione firmware alta, bassa, ...) – i 6 byte di risposta a `F0`.
pub fn controller_info<P: Read + Write>(p: &mut P) -> Result<[u8; 6], String> {
    send_echo(p, &[0xF0])?;
    let mut info = [0u8; 6];
    for slot in info.iter_mut() {
        p.write_all(&[0]).map_err(|e| e.to_string())?;
        *slot = read_byte(p)?;
    }
    if info[0] == 0 {
        return Err("Controller non riconosciuto".into());
    }
    Ok(info)
}

pub const FAST_BAUD: u32 = 115_200;

pub fn supported_controller(info: [u8; 6]) -> Result<(), String> {
    // Tipo 02 osservato sul MR-C3024 reale; gli altri tipi non sono verificati.
    if info[0] != 2 {
        return Err(format!(
            "Controller non supportato (tipo {:02X}); richiesto MR-C3024",
            info[0]
        ));
    }
    Ok(())
}

pub fn servo<P: Read + Write>(p: &mut P, motor: u8, pos: u8) -> Result<(), String> {
    if motor >= 24 || !(10..=190).contains(&pos) {
        return Err("Servo fuori intervallo: motore 0-23, posizione 10-190".into());
    }
    let cmd = [0xE6, motor, pos];
    p.write_all(&cmd).map_err(|e| e.to_string())?;
    let mut echo = [0; 3];
    p.read_exact(&mut echo)
        .map_err(|e| format!("Lettura eco servo: {e}"))?;
    if echo != cmd {
        return Err(format!("Eco servo errato: {echo:02X?}"));
    }
    Ok(())
}

/// `title`: caratteri del nome da mostrare sul controller (8, riempiti con spazi).
pub fn download<P: Read + Write>(
    p: &mut P,
    object: &[u8],
    title: &str,
    fast: &mut dyn FnMut(&mut P) -> Result<(), String>,
    progress: &mut dyn FnMut(usize, usize),
) -> Result<(), String> {
    let img = image(object)?;
    supported_controller(controller_info(p)?)?;
    // F7 aa bb + byte nullo: il controller risponde con un valore (non un eco)
    for cmd in [[0xF7, 0xF2, 0x04], [0xF7, 0xF3, 0x04]] {
        send_echo(p, &cmd)?;
        p.write_all(&[0]).map_err(|e| e.to_string())?;
        read_byte(p)?;
    }
    send_echo(p, &[0xFA])?;
    let mut name: Vec<u8> = title.bytes().take(8).collect();
    name.resize(8, b' ');
    for (i, c) in name.iter().enumerate() {
        send_echo(p, &[0xB4, 0x4B, i as u8, 0x00, *c, *c])?;
    }
    send_echo(p, &[0xEC])?;
    // dopo EC il controller passa a una velocita' maggiore; la lunghezza ha eco, i blocchi no
    fast(p)?;
    send_echo(p, &[img.len() as u8, (img.len() >> 8) as u8])?;
    let all = blocks(&img);
    for (n, b) in all.iter().enumerate() {
        p.write_all(&[0x3F]).map_err(|e| e.to_string())?;
        match read_byte(p)? {
            0x21 => {}
            0x45 => return Err("Il controller ha interrotto il download".into()),
            r => return Err(format!("Risposta inattesa del controller: {r:02X}")),
        }
        p.write_all(b).map_err(|e| e.to_string())?;
        progress(n + 1, all.len());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // Scrive sul robot reale: ROBO_PORT=COM1 cargo test real_beep -- --ignored --nocapture
    #[test]
    #[ignore]
    fn real_beep() {
        let name = std::env::var("ROBO_PORT").expect("Impostare ROBO_PORT esplicitamente");
        assert_eq!(
            std::env::var("ROBO_ALLOW_WRITE").as_deref(),
            Ok("YES"),
            "Scrittura sul robot: impostare ROBO_ALLOW_WRITE=YES dopo il consenso"
        );
        let mut p = serialport::new(name, 9_600)
            .timeout(std::time::Duration::from_millis(2000))
            .open()
            .unwrap();
        let obj = crate::compiler::compile("BEEP\r\nEND\r\n", "BEEPTEST", [26, 10, 26, 18, 0, 0])
            .unwrap();
        let r = download(
            &mut p,
            &obj,
            "BEEPTEST",
            &mut |p| p.set_baud_rate(FAST_BAUD).map_err(|e| e.to_string()),
            &mut |n, t| println!("blocco {n}/{t}"),
        );
        println!("{r:?}");
        r.unwrap();
    }

    #[test]
    #[ignore]
    fn real_music() {
        let name = std::env::var("ROBO_PORT").expect("Impostare ROBO_PORT esplicitamente");
        assert_eq!(
            std::env::var("ROBO_ALLOW_WRITE").as_deref(),
            Ok("YES"),
            "Scrittura sul robot: impostare ROBO_ALLOW_WRITE=YES dopo il consenso"
        );
        let mut p = serialport::new(name, 9_600)
            .timeout(std::time::Duration::from_millis(2000))
            .open()
            .unwrap();
        let obj = crate::compiler::compile(
            "MUSIC \"CDE\"\r\nEND\r\n",
            "MUSICTST",
            [31, 10, 26, 0, 0, 0],
        )
        .unwrap();
        download(
            &mut p,
            &obj,
            "MUSICTST",
            &mut |p| p.set_baud_rate(FAST_BAUD).map_err(|e| e.to_string()),
            &mut |n, t| println!("blocco {n}/{t}"),
        )
        .unwrap();
        println!("MUSIC + END inviato; sessione chiusa al termine del test, nessun comando servo");
    }

    // Sola lettura sul robot reale: ROBO_PORT=COM1 cargo test real_info -- --ignored --nocapture
    #[test]
    #[ignore]
    fn real_info() {
        let name = std::env::var("ROBO_PORT").expect("Impostare ROBO_PORT esplicitamente");
        let mut p = serialport::new(name, 9_600)
            .timeout(std::time::Duration::from_millis(1000))
            .open()
            .unwrap();
        let info = controller_info(&mut p).unwrap();
        supported_controller(info).unwrap();
        println!("{info:02X?}");
    }

    // Controller finto: eco dei comandi, `3F` -> `21`, 129 byte di blocco assorbiti senza risposta.
    struct Fake {
        out: VecDeque<u8>,
        info: VecDeque<u8>,
        rx: Vec<u8>,
        pending_data: usize,
        reading_info: bool,
        block_reply: u8,
        echo_error: bool,
    }

    impl Read for Fake {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            match self.out.pop_front() {
                Some(b) => {
                    buf[0] = b;
                    Ok(1)
                }
                None => Ok(0),
            }
        }
    }

    impl Write for Fake {
        fn write(&mut self, data: &[u8]) -> std::io::Result<usize> {
            for &b in data {
                self.rx.push(b);
                if self.pending_data > 0 {
                    self.pending_data -= 1;
                } else if self.reading_info {
                    if let Some(i) = self.info.pop_front() {
                        self.out.push_back(i);
                    }
                    if self.info.is_empty() {
                        self.reading_info = false;
                    }
                } else if b == 0x3F {
                    self.out.push_back(self.block_reply);
                    self.pending_data = 129;
                } else {
                    self.out.push_back(if self.echo_error { b ^ 1 } else { b });
                    if b == 0xF0 {
                        self.reading_info = true;
                    }
                }
            }
            Ok(data.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn checksum_matches_original_ide() {
        // Blocco catturato da roboBASIC v2.72 con `DIM A AS BYTE / A = 1 / DELAY 100 / END`.
        let obj = [
            0x00, b'U', b'n', b't', b'i', b't', b'l', b'e', b'd', 0x1A, 0x0A, 0x1A, 0x0F, 0x04,
            0x1F, 0x1F, 0x00, 0xD0, 0x15, 0x40, 0x11, 0x00, 0xD4, 0x12, 0x64, 0xE9, 0xC4, 0x19,
            0x00, 0xC4, 0x1C, 0x00,
        ];
        let img = image(&obj).unwrap();
        assert_eq!(img.len(), 31);
        assert_eq!(&img[..8], b"\xFFUntitle");
        let b = blocks(&img);
        assert_eq!(b.len(), 1);
        assert_eq!(b[0].len(), 129);
        assert_eq!(b[0][128], 0x99);
    }

    #[test]
    fn oversized_objects_are_rejected_before_serial_commands() {
        let mut f = fake();
        assert!(
            download(
                &mut f,
                &vec![0; 65_537],
                "TEST",
                &mut |_| Ok(()),
                &mut |_, _| {}
            )
            .unwrap_err()
            .contains("troppo grande")
        );
        assert!(f.rx.is_empty());
    }

    #[test]
    fn controller_capacity_and_object_length_are_checked_before_io() {
        let source = format!("PRINT \"{}\"\nEND", "A".repeat(MAX_PROGRAM_BYTES - 25));
        let object = crate::compiler::compile(&source, "", [0; 6]).unwrap();
        let img = image(&object).unwrap();
        assert_eq!(img.len(), MAX_PROGRAM_BYTES);
        assert_eq!(blocks(&img).len(), MAX_PROGRAM_BYTES / BLOCK);

        let too_large =
            crate::compiler::compile(&source.replace("PRINT \"", "PRINT \"A"), "", [0; 6]).unwrap();
        let mut bad_length = crate::compiler::compile("END", "", [0; 6]).unwrap();
        bad_length[15] = 0;
        for (object, message) in [(too_large, "32768"), (bad_length, "incoerente")] {
            let mut f = fake();
            let error = download(
                &mut f,
                &object,
                "TEST",
                &mut |_| panic!("Nessun cambio baud per un oggetto invalido"),
                &mut |_, _| panic!("Nessun trasferimento per un oggetto invalido"),
            )
            .unwrap_err();
            assert!(error.contains(message), "{error}");
            assert!(f.rx.is_empty());
        }
    }

    #[test]
    fn download_sequence() {
        let mut f = Fake {
            out: VecDeque::new(),
            info: VecDeque::from(vec![2, 2, 7, 0x40, 0xBE, 5]),
            rx: vec![],
            pending_data: 0,
            reading_info: false,
            block_reply: 0x21,
            echo_error: false,
        };
        let obj = crate::compiler::compile(
            "DIM A AS BYTE\r\nA = 1\r\nDELAY 100\r\nEND\r\n",
            "t",
            [26, 10, 26, 15, 4, 31],
        )
        .unwrap();
        let mut steps = vec![];
        download(&mut f, &obj, "d1.bas", &mut |_| Ok(()), &mut |n, t| {
            steps.push((n, t))
        })
        .unwrap();
        assert_eq!(steps, vec![(1, 1)]);
        assert_eq!(&f.rx[..7], &[0xF0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&f.rx[7..15], &[0xF7, 0xF2, 4, 0, 0xF7, 0xF3, 4, 0]);
        assert_eq!(f.rx.iter().filter(|&&b| b == 0xB4).count(), 8);
        assert_eq!(f.rx.len(), 7 + 9 + 48 + 3 + 1 + 129);
        let title: Vec<_> = f.rx[16..64].chunks(6).map(|c| c[4]).collect();
        assert_eq!(&title, b"d1.bas  ");
    }

    fn fake() -> Fake {
        Fake {
            out: VecDeque::new(),
            info: VecDeque::from([2, 2, 7, 0x40, 0xBE, 5]),
            rx: vec![],
            pending_data: 0,
            reading_info: false,
            block_reply: 0x21,
            echo_error: false,
        }
    }

    #[test]
    fn servo_requires_complete_matching_echo() {
        let mut f = fake();
        servo(&mut f, 23, 190).unwrap();
        assert_eq!(f.rx, [0xE6, 23, 190]);
        assert!(f.out.is_empty());
        f.echo_error = true;
        assert!(servo(&mut f, 0, 100).unwrap_err().contains("Eco"));
        let mut empty = std::io::Cursor::new(Vec::new());
        assert!(servo(&mut empty, 0, 100).is_err());
        for (m, p) in [(24, 100), (0, 9), (0, 191)] {
            let mut f = fake();
            assert!(servo(&mut f, m, p).is_err());
            assert!(f.rx.is_empty());
        }
    }

    #[test]
    fn rejects_unverified_controller_and_bad_block() {
        let obj = crate::compiler::compile("END", "TEST", [0; 6]).unwrap();
        let mut f = fake();
        f.info[0] = 4;
        assert!(
            download(&mut f, &obj, "TEST", &mut |_| Ok(()), &mut |_, _| {})
                .unwrap_err()
                .contains("non supportato")
        );
        assert_eq!(f.rx.len(), 7);
        let mut f = fake();
        f.block_reply = 0x45;
        assert!(
            download(&mut f, &obj, "TEST", &mut |_| Ok(()), &mut |_, _| {})
                .unwrap_err()
                .contains("interrotto")
        );
        let mut f = fake();
        assert!(
            download(
                &mut f,
                &obj,
                "TEST",
                &mut |_| Err("Cambio baud fallito".into()),
                &mut |_, _| {}
            )
            .unwrap_err()
            .contains("baud")
        );
        assert_eq!(f.rx.last(), Some(&0xEC));
    }

    #[test]
    fn multiple_blocks_follow_baud_switch() {
        let src = "DELAY 100\n".repeat(80) + "END";
        let obj = crate::compiler::compile(&src, "TEST", [0; 6]).unwrap();
        let expected = blocks(&image(&obj).unwrap());
        let mut f = fake();
        let mut switched = false;
        let mut progress = vec![];
        download(
            &mut f,
            &obj,
            "TEST",
            &mut |f| {
                assert_eq!(f.rx.last(), Some(&0xEC));
                switched = true;
                Ok(())
            },
            &mut |n, t| progress.push((n, t)),
        )
        .unwrap();
        assert!(switched);
        assert_eq!(progress.len(), expected.len());
        assert_eq!(progress.last(), Some(&(expected.len(), expected.len())));
        for (received, block) in f.rx[67..].chunks(130).zip(expected) {
            assert_eq!(received[0], 0x3F);
            assert_eq!(&received[1..], block);
        }
    }
}
