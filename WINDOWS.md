# Istruzioni per Copilot su Windows (RoboZakBasic)

Contesto: riscrittura in Rust/Tauri v2 di **roboBASIC v2.72** (IDE VB6 per Robonova/MRC-3000). Rispondi in italiano, in modo conciso.
Leggi prima `re/NOTES.md` (stato del reverse engineering) e `README`/codice in `robozak/`.

## Stato
- Fatti: parser/interprete BASIC con simulatore (`robozak/src-tauri/src/basic.rs`), comandi Tauri e seriale grezza (`lib.rs`), UI con editor, sliders, console e help in italiano.
- Bloccato: il compilatore BASIC -> binario per il firmware e il protocollo di download/controllo live. Gli originali `rbv2.ocx` e `roboRemocon.exe` sono **VB6 p-code**: Ghidra non li decompila e VBDecompiler (Zig) non è affidabile.

## Setup
1. `gh repo clone rsodvd79/RoboZakBasic`
2. Installa Rust (rustup), Node LTS, WebView2 (già in Windows 11) e Visual Studio Build Tools (C++).
3. `cd robozak && npm install && npm run tauri dev`
4. Test Rust: `cd robozak/src-tauri && cargo test`
5. Installa roboBASIC dall'MSI `roboBASIC v2.72.msi` (non è nel repo: copialo a mano). Estrai anche una copia dei file in `extracted/` (gitignored) con `msiexec /a "roboBASIC v2.72.msi" /qb TARGETDIR=C:\rb_extract`.

## Compiti, in ordine
1. **Decompilare il p-code**: usa VB Decompiler Lite o VBDec (sandsprite) su `rbv2.ocx` (compilatore: metodi `Compile`, `TotalBytes`, `Bas_Error*`) e su `roboRemocon.exe`. Salva l'output in `re/decomp/`. Le stringhe UTF-16 sono già in `re/scripts/u16strings.py`.
2. **Compilatore come black box**: con roboBASIC originale compila programmi BASIC minimi (es. `MOTOR 0`, `DELAY 100`, `END`) e salva i binari/hex prodotti. Varia un'istruzione alla volta per dedurre opcode, operandi, tabella delle label (max 255), header e checksum. Salva i casi in `re/samples/` (`.bas` + output).
3. **Sniffing seriale** (115200, 8N1, MSComm): con Robonova collegato catturare con Portmon/Device Monitor/Serial Port Monitor il download di un programma, il remote-control (`rrm_header_pc.bas`, `ON A GOTO MAIN_LOOP,K1..K32`) e l'aggiornamento firmware. Salva i dump in `re/captures/` con una nota su cosa è stato fatto.
4. **Implementare** in Rust: `lang-compiler` (prima MRC-3000, poi MRC-2000/MA2K/MA3K), nuovo comando Tauri di download, controllo live dei servo (gli slider oggi muovono solo la vista) e selettore del controller.
5. **Verificare** sul robot reale confrontando gli output con quelli dell'originale (byte per byte).
6. Aggiorna `re/NOTES.md`, committa e fai push a piccoli passi.

## Note tecniche
- File `.cmd`: CSV `ENGLISH,korean` in CP949 (`*e.cmd` solo inglese).
- Gruppi motori: G6A..G6D = motori 0-23 (6 ciascuno), G8x = 8, G12/16/24/32 partono da 0. Angolo servo ~10-190 (default 100).
- MR-C2000: 30 byte di variabili; MR-C3000: 256. Pseudo-comandi `'$DEVICE` e `'$LIMIT`.
- Tauri converte gli argomenti snake_case Rust in camelCase in JS.
- Non committare gli originali Microsoft/Hanool (`extracted/`, `chm/`, MSI): sono in `.gitignore`.
