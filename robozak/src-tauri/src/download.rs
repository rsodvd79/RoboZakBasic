//! Download del programma compilato sul controller (protocollo ricavato da roboBASIC v2.72,
//! vedi re/NOTES.md). Il controller fa l'eco di ogni byte dei comandi; `3F` e i blocchi dati no.

use std::io::{Read, Write};

const BLOCK: usize = 128;

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
    if object.len() < 17 {
        return Err("Oggetto compilato troppo corto".into());
    }
    let mut v = vec![0xFF];
    v.extend_from_slice(&object[1..8]);
    v.extend_from_slice(&object[9..]);
    Ok(v)
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

/// `title`: caratteri del nome da mostrare sul controller (8, riempiti con spazi).
pub fn download<P: Read + Write>(
    p: &mut P,
    object: &[u8],
    title: &str,
    progress: &mut dyn FnMut(usize, usize),
) -> Result<(), String> {
    let img = image(object)?;
    controller_info(p)?;
    send_echo(p, &[0xF7, 0xF2, 0x04, 0x00, 0xF7, 0xF3, 0x04, 0x00, 0xFA])?;
    let mut name: Vec<u8> = title.bytes().take(8).collect();
    name.resize(8, b' ');
    for (i, c) in name.iter().enumerate() {
        send_echo(p, &[0xB4, 0x4B, i as u8, 0x00, *c, *c])?;
    }
    send_echo(p, &[0xEC, img.len() as u8, (img.len() >> 8) as u8])?;
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

    // Controller finto: eco dei comandi, `3F` -> `21`, 129 byte di blocco assorbiti senza risposta.
    struct Fake {
        out: VecDeque<u8>,
        info: VecDeque<u8>,
        rx: Vec<u8>,
        pending_data: usize,
        reading_info: bool,
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
                    self.out.push_back(0x21);
                    self.pending_data = 129;
                } else {
                    self.out.push_back(b);
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
    fn download_sequence() {
        let mut f = Fake {
            out: VecDeque::new(),
            info: VecDeque::from(vec![4, 2, 1, 0, 0, 0]),
            rx: vec![],
            pending_data: 0,
            reading_info: false,
        };
        let obj = crate::compiler::compile("DIM A AS BYTE\r\nA = 1\r\nDELAY 100\r\nEND\r\n", "t", [26, 10, 26, 15, 4, 31]).unwrap();
        let mut steps = vec![];
        download(&mut f, &obj, "d1.bas", &mut |n, t| steps.push((n, t))).unwrap();
        assert_eq!(steps, vec![(1, 1)]);
        assert_eq!(&f.rx[..7], &[0xF0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&f.rx[7..15], &[0xF7, 0xF2, 4, 0, 0xF7, 0xF3, 4, 0]);
        assert_eq!(f.rx.iter().filter(|&&b| b == 0xB4).count(), 8);
        assert_eq!(f.rx.len(), 7 + 9 + 48 + 3 + 1 + 129);
    }
}
