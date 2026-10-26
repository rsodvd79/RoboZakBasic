# Note di reverse engineering (roboBASIC v2.72)

## Trovato
- App VB6 (MSVBVM60). Compilatore BASIC in `rbv2.ocx` (metodi `Compile`, `TotalBytes`, `Bas_Error*`), usato da `rbv2.exe` e `roboRemocon.exe`.
- Seriale: `115200,n,8,1` (MSComm32), download del programma compilato + verifica codici, controllo/aggiornamento firmware.
- Tabelle keyword/sintassi e messaggi d'errore del compilatore sono stringhe UTF-16 in `rbv2.ocx` (`re/scripts/u16strings.py`).
- Comandi per controller: `extracted/mrc*.cmd` (CP949); modelli robot: `extracted/user/*.dat`.

## Bloccato
- `rbv2.ocx` / `roboRemocon.exe` sono VB6 **p-code**: Ghidra (12.2, `re/scripts/DecompileAll.java`, serve JDK 21+) vede solo ~24 funzioni di runtime, non il codice delle procedure.
- Formato binario compilato e protocollo di download/PTP live non ricavabili staticamente senza un disassemblatore p-code (es. VB Decompiler) o lo sniffing della seriale con roboBASIC originale + Robonova.

## Strumenti provati
- Ghidra 12.2: nessun supporto p-code VB6.
- DaCodeChick/VBDecompiler (compilato con Zig 0.16 + patch a `SQLITE_TRANSIENT` in `core/src/db/sqlite.zig`): riconosce `rbv2.ocx` come p-code ma non trova la VB Object Table, e la tabella opcode in `pcode_opcodes.zig` è inventata (non corrisponde a MSVBVM60): inutilizzabile per il nostro scopo.
- Restano: VBDec (sandsprite.com/vbdec) o VB Decompiler Lite su Windows/VM; oppure eseguire roboBASIC originale (Windows/Wine) come riferimento black-box.

## Compilatore come black box (sbloccato)
`rbv2.ocx` è un controllo COM registrato (`RBv2.ROBOBASICv2`, oggi punta a `roboBASIC MF v2.80`, stesso formato). Da PowerShell **32 bit**: `Compile(<testo sorgente>)` (non un path!) poi `ObjCode` (byte), `Bas_Error*`, `TotalBytes`.
- `re/scripts/rbcompile.ps1 <file|cartella>` compila `.bas` -> `.hex`.
- `re/scripts/rbcases.ps1` compila i casi di `re/samples/cases.txt` (`nome = riga|riga`) e salva `.bas`/`.hex` (149 campioni).
- Implementazione Rust: `robozak/src-tauri/src/compiler.rs`; il test `matches_original_compiler` confronta byte per byte con i `.hex` (ignorando il timestamp).

### Formato del binario (offset dall'inizio del file)
- `0`: `00`; `1..8`: nome (8 byte, riempito con spazi); `9..14`: timestamp compilazione (giorno, mese, anno%100, ore, minuti, secondi); `15..16`: lunghezza LE = totale-1; `17..`: codice.
- Gli indirizzi (2 byte LE) sono offset-1 (il primo byte di codice è `0x10`). Ogni programma termina con un `C4 <self>` aggiunto automaticamente. Nessun checksum nel buffer (da verificare nel frame seriale).
- Numeri: `10`=0, `11`=1, `12 nn`=2..255, `13 lo hi`=256..65535. Negativi, parentesi, `NOT` e array: non supportati dal compilatore. Stringhe: `18 <ascii> 00`.
- Variabili: da `0x40`; BYTE tag `15 addr` (1 byte), INTEGER tag `16 addr` (2 byte). Nessuna precedenza: espressioni valutate in sequenza (`A + 1 * 2` = `(A+1)*2`).
- Operatori: `+21 -22 *23 /24 MOD26 AND27 OR28 XOR2A <<2C >>2D <30 >31 <=32 >=33 <>34 =35`.
- Funzioni: `RND F8`, `IN F0`, `BYTEIN F2`, `AD F3`, `PEEK F9`, `REMOCON FB`, `SONAR FD`.
- Istruzioni: `C0 else cond 00`=IF (blocco; ELSE/ELSEIF emettono `C4 fine`), `C1 var inizio`/`C3 var limite uscita`/`C2 var C3addr`=FOR/NEXT (nessun STEP), `CB` BREAK, `C4` GOTO (anche END = salto a sé), `C5` GOSUB, `C6` RETURN, `C7 op n addr..` ON GOTO, `D0 var expr 00` assegnazione, `D1` OUT, `D3` TOGGLE, `D4 op E9` DELAY, `DA`/`DB` MOTOR/MOTOROFF (uno per motore), `DD` SPEED, `DF` SERVO, `B0 start n v..` MOVE, `B4 start n mask` DIR, `B9 start n flag..` GETMOTORSET, `ED 01/00` PTP SETON/OFF, `B6 06/05` PTP ALLON/OFF, `B6 01/00` HIGHSPEED, `E0..E4` LCDINIT/CLS/LOCATE/PRINT/CSON, `E5` CSOFF, `E6` CONT, `E7` BYTEOUT, `E9` WAIT, `EA E9` STOP, `CC` POKE, `BF` TEMPO.
- Gruppi (start,n): G6A 0,6 · G6B 6,6 · G6C 12,6 · G6D 18,6 · G6E 24,6 · G8A 0,8 · G12/16/24/32 da 0. MOVE: valori solo costanti, omessi = 0.
- Non ancora gestiti: BEEP, MUSIC, PWM/FPWM, ETX/ERX, GYRO*/ACCEL*/TILT*, AIMOTOR, array, STEP, RCIN, MOVE G8B..G8D.

## Decompilazione (Windows)
- **Correzione**: `rbv2.ocx`, `rbv2.exe` (e probabilmente `roboRemocon.exe`) sono VB6 **codice nativo x86** (non p-code). I tentativi precedenti (macOS) li avevano classificati male.
- VB Decompiler Lite 26.5 (installato in `C:\rb_tools\vbd`) apre i file e esporta il disassembly con i nomi delle procedure (`File > Salva tutto in un modulo` -> `re/decomp/rbv2_all.bas`, 8,7 MB, ignorato da git). La Lite non produce VB ad alto livello; le chiamate a MSComm sono late-bound (`__vbaLateIdCallLd`) quindi i nomi dei metodi non compaiono.
- Procedure di interesse in `rbv2.exe`: `frmMain.Comm_OnComm` (0x58F600), `mnuConRun_Click` (0x5969C0), `mnuAllRun_Click` (0x5964D0).

## Prossimi passi
1. Analizzare in Ghidra (x86 nativo) `rbv2.exe`/`roboRemocon.exe` a partire dalle procedure sopra, oppure catturare la seriale con una coppia COM virtuale. con nuovi casi in `cases.txt` (stesso metodo).
2. Catturare il traffico seriale di roboBASIC (download di un `.bas` e controllo servo live): resta da scoprire il framing del download e l'eventuale checksum.
3. Esporre `compiler::compile` come comando Tauri di download e implementare l'invio live dei servo in `serial_write`.
