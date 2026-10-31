# Note di reverse engineering (roboBASIC v2.72)

**Stato: MR-C3024 (MRC-3000) implementato nel sottoinsieme verificato** (compilatore, download e controllo live con prove storiche su robot reale). Nuovo collaudo autorizzato su COM1: `real_info` risponde `02 02 07 40 BE 05`; `real_music` trasferisce MUSIC + END; una successiva riapertura a 9600 baud risponde correttamente. Nessun movimento servo è stato inviato: il nuovo collaudo fisico dei servo resta da autorizzare. Gli altri controller non sono implementati.

## Trovato
- App VB6 (MSVBVM60). Compilatore BASIC in `rbv2.ocx` (metodi `Compile`, `TotalBytes`, `Bas_Error*`), usato da `rbv2.exe` e `roboRemocon.exe`.
- Seriale (MSComm32): handshake a 9600,n,8,1, trasferimento dati a 115200 (vedi "Robot reale"). Il robot reale non risponde a 115200 prima del passaggio.
- Tabelle keyword/sintassi e messaggi d'errore del compilatore sono stringhe UTF-16 in `rbv2.ocx` (`re/scripts/u16strings.py`).
- Comandi per controller: `extracted/mrc*.cmd` (CP949); modelli robot: `extracted/user/*.dat`.

## Metodo
Gli eseguibili sono VB6 a codice nativo (x86, non p-code); la decompilazione statica ha reso poco (VB Decompiler Lite apre i file, Ghidra vede solo il runtime). Ha funzionato: (1) usare `rbv2.ocx` come black box per il compilatore, (2) Frida sull'IDE originale per catturare la seriale (`re/scripts/frida_serial.py`), (3) com0com + `re/scripts/fakerobot.py` per un robot finto nei test, (4) Ghidra (`re/scripts/DecompileAll.java`, JDK 21+) solo di supporto.

## Compilatore come black box (sbloccato)
`rbv2.ocx` è un controllo COM registrato (`RBv2.ROBOBASICv2`, oggi punta a `roboBASIC MF v2.80`, stesso formato). Da PowerShell **32 bit**: `Compile(<testo sorgente>)` (non un path!) poi `ObjCode` (byte), `Bas_Error*`, `TotalBytes`.
- `re/scripts/rbcompile.ps1 <file|cartella>` compila `.bas` -> `.hex`.
- `re/scripts/rbcases.ps1` compila i casi di `re/samples/cases.txt` (`nome = riga|riga`) e salva `.bas`/`.hex`. Il corpus attuale ha 274 positivi e 78 negativi: 352 sorgenti verificati, non tutti i casi esplorativi elencati in `cases.txt`.
- `re/scripts/rbcompile.ps1` con `-ExpectationsPath` registra in JSON nome del campione, codice d'errore e riga dall'OCX (default `ConType=4`, `ConID=2`), senza modificare gli `.hex`. `re/samples/expectations.json` è stato ricavato dall'originale per tutti i sorgenti esistenti.
- Implementazione Rust: `robozak/src-tauri/src/compiler.rs`; il test `matches_original_compiler` confronta byte per byte con i `.hex` (ignorando il timestamp). Ora fallisce anche per inventario incoerente, campioni non supportati, negativi accettati o righe d'errore diverse. Non confronta il testo originale dei messaggi con quello italiano della UI.

### Formato del binario (offset dall'inizio del file)
- `0`: `00`; `1..8`: nome (8 byte, riempito con spazi); `9..14`: timestamp compilazione (giorno, mese, anno%100, ore, minuti, secondi); `15..16`: lunghezza LE = totale-1; `17..`: codice.
- Gli indirizzi (2 byte LE) sono offset-1 (il primo byte di codice è `0x10`). Ogni programma termina con un `C4 <self>` aggiunto automaticamente. Nessun checksum nel buffer (da verificare nel frame seriale).
- Numeri: `10`=0, `11`=1, `12 nn`=2..255, `13 lo hi`=256..65535; accettate anche le basi `&H`, `&B`, `&O`. Negativi, parentesi di raggruppamento e array restano esclusi. Stringhe PRINT: `18 <ascii> 00`; nell'assegnazione viene usato il primo carattere ASCII.
- Variabili: da `0x40`; BYTE tag `15 addr` (1 byte), INTEGER tag `16 addr` (2 byte). INTEGER unsigned 0-65535. Aritmetica/bit sequenziali (`A + 1 * 2` = `(A+1)*2`), poi confronti e combinazioni logiche di confronti nel simulatore; la sequenza degli operatori nel bytecode non implica assenza di precedenza tra classi.
- Bit: `1B addr mask`, dove addr = indirizzo variabile + bit/8 e mask = 1<<(bit%8). L'OCX accetta anche bit 8-15 su BYTE e scrive la variabile successiva; Rust rifiuta questi accessi, conformemente al manuale.
- Operatori: `+21 -22 *23 /24 MOD26 AND27 OR28 XOR2A <<2C >>2D <30 >31 <=32 >=33 <>34 =35`.
- Funzioni: `RND F8`, `IN F0`, `KEYIN F1` (due operandi), `NOT EF` (un operando, sintassi NOT(x)), `BYTEIN F2`, `AD F3`, `PEEK F9`, `REMOCON FB`, `SONAR FD`.
- `PULSE op` = `D2`; `RUN` = `EB`. `POS/ZERO/INIT g,v...` = `B1/B3/B5 start n valori`; intervalli dei valori espliciti: POS/INIT 10-190, ZERO 80-120. I valori omessi producono 00, mentre uno 0 esplicito viene rifiutato.
- `PRINT "OK",65` = `E3 18 4F 4B 41 00`: i numeri sono codici carattere 1-255, non valori da visualizzare in decimale. FORMAT = `E3 tag formato tag variabile addr cifre`, senza terminatore 00; formato HEX=19, DEC=1A, BIN=1B. Cifre predefinite BYTE/INTEGER: HEX 2/4, DEC 3/5, BIN 8/16; cifre esplicite 1-255.
- MOVEPOS viene espanso in MOVE, con pose dichiarate mediante etichette POS, anche inline e successive al richiamo. L'OCX attuale rifiuta tutte le forme MOVEPOS provate: si confronta quindi il bytecode del sorgente espanso, non un ipotetico opcode B2.
- Istruzioni: `C0 else cond 00`=IF (blocco; ELSE/ELSEIF emettono `C4 fine`), `C1 var inizio`/`C3 var limite uscita`/`C2 var C3addr`=FOR/NEXT (nessun STEP), `CB` BREAK, `C4` GOTO (anche END = salto a sé), `C5` GOSUB, `C6` RETURN, `C7 op n addr..` ON GOTO, `D0 var expr 00` assegnazione, `D1` OUT, `D3` TOGGLE, `D4 op E9` DELAY, `DA`/`DB` MOTOR/MOTOROFF (uno per motore), `DD` SPEED, `DF` SERVO, `B0 start n v..` MOVE, `B4 start n mask` DIR, `B9 start n flag..` GETMOTORSET, `ED 01/00` PTP SETON/OFF, `B6 06/05` PTP ALLON/OFF, `B6 01/00` HIGHSPEED, `E0..E4` LCDINIT/CLS/LOCATE/PRINT/CSON, `E5` CSOFF, `E6` CONT, `E7` BYTEOUT, `E9` WAIT, `EA E9` STOP, `CC` POKE, `BF` TEMPO.
- Gruppi (start,n): G6A 0,6 · G6B 6,6 · G6C 12,6 · G6D 18,6 · G6E 24,6 · G8A 0,8 · G12/16/24/32 da 0. MOVE: valori solo costanti, omessi = 0.
- Altri gruppi: G8B 8,8 · G8C 16,8 · G8D 24,8.
- Funzioni con un argomento: `ROMPEEK FA`, `MOTORIN FE`, `STATE F6`, `AIMOTORIN FC`. `RCIN(n)` = `F3 12 (100+n)`.
- `ACTION n` (solo costante) = `D0 15 40 12 (0x32+n) 00 C5 28 27`; `FUNC n` = `B6 n`; `BEEP` = `CF 14 02 00` (nessun argomento); `PWM a,b` = `DE`; `FPWM a,b,c` = `EE`; `ROMPOKE a,b` = `CD`.
- `AIMOTOR n` = `B7`, `AIMOTOROFF n` = `B8`; i gruppi espandono un'istruzione per motore. AIMOTOR SETOFF/SETON/INIT = `B6 02/03/04`; AIMOTOR ALLON e AIMOTOROFF ALLOFF espandono 31 canali (0-30), MOTOR ALLON/MOTOROFF ALLOFF 32 canali del formato (non altrettanti motori live MR-C3024).
- `GYROSET/GYRODIR/GYROSENSE g,v1..v6` = `A0/A1/A2 start n` + 6 valori. `ACCELSET`/`TILTSET` = `A0`/`A9 00 20` + 32 byte (primo argomento scartato). `ACCELDIR/SENSE`, `TILTDIR/SENSE` = `00 20` + 32 byte (6 valori all'inizio, resto 0).
- `ETX baud,x` = `D6 <codice> op`; `ERX baud,var,label` = `D5 <codice> <tag var> <addr label>`. Codici baud: 2400=0, 4800=1, 9600=2, 14400=3, 19200=4, 28800=5, 38400=6, 57600=7, 76800=8, 115200=9, 230400=10.
- `MUSIC "..."` = `BA <addr> <coppie durata,nota> 00`. Ottava 3 (default): C=0x24, D=26, E=28, F=29, G=2B, A=2D, B=2F; +12 per ottava. Durata `Ln` = 96/n (default L4 = 0x18); `Ln` riporta l'ottava a 2; `>`/`<` ±1 ottava; `#`/`+`/`-` alterano di 1 semitono; `P` = pausa `7F`; `H` ignorato; `R`, `T`, `V`, `.` = errore 47.
- `FOR..STEP`, NOT senza parentesi, KEYIN con un solo argomento e POS come funzione sono rifiutati dall'originale. I vecchi campioni negativi ZERO/INIT non provavano l'assenza delle istruzioni: avevano argomenti fuori intervallo.
- Limiti: oltre 192 byte di variabili (0x40..0xFF) l'originale non da' errore ma produce un programma vuoto (`END` senza codice); nessun limite sul numero di label (provati 300).

## Decompilazione (Windows)
- **Correzione**: `rbv2.ocx`, `rbv2.exe` (e probabilmente `roboRemocon.exe`) sono VB6 **codice nativo x86** (non p-code). I tentativi precedenti (macOS) li avevano classificati male.
- VB Decompiler Lite 26.5 (installato in `C:\rb_tools\vbd`) apre i file e esporta il disassembly con i nomi delle procedure (`File > Salva tutto in un modulo` -> `re/decomp/rbv2_all.bas`, 8,7 MB, ignorato da git). La Lite non produce VB ad alto livello; le chiamate a MSComm sono late-bound (`__vbaLateIdCallLd`) quindi i nomi dei metodi non compaiono.
- Procedure di interesse in `rbv2.exe`: `frmMain.Comm_OnComm` (0x58F600), `mnuConRun_Click` (0x5969C0), `mnuAllRun_Click` (0x5964D0).

## Robot reale (verificato su MRC-3000, COM1)
Ogni byte inviato viene normalmente rimandato come eco; dove il controller risponde con un dato e' indicato.
- Handshake a 9600 baud: `F0` + 6 byte `00` (ogni `00` riceve un byte di info; MR-C3024: `02 02 07 40 BE 05`), `F7 F2 04 00` e `F7 F3 04 00` (il 4o byte riceve un dato, non un eco), `FA`, nome del programma `B4 4B i 00 c c` (8 caratteri), poi `EC`.
- Dopo `EC` il controller passa a 115200 baud: lunghezza (2 byte LE, con eco), poi per ogni blocco da 128 byte `3F` -> `21` e 129 byte senza eco (128 dati zero-padded + checksum somma mod 256). Il buffer e' `FF`, nome (7 char), data (6), lunghezza (2 LE), codice. Nel log dell'IDE non c'e' una conferma finale dopo l'ultimo blocco.
- Controllo live servo: `E6 <motore> <pos>` (eco dei 3 byte). La lettura posizioni usa `F7 (EB+n/8) 04 00`, `F7 (CA+n/8) 04 00`, `F7 n 03 00` (risposta = posizione), `D2 n 00`, `E7 n`.
- Implementazione: `robozak/src-tauri/src/download.rs` (`FAST_BAUD`), comandi Tauri `download_program` e `servo_set` in `lib.rs`.
- Identificazione attuale: tipo informativo `02`, osservato sul MR-C3024 reale; altri tipi vengono rifiutati prima del download. Questo non estende il supporto agli altri controller.
- La connessione identifica il robot a 9600 baud. Il controllo servo convalida l'eco completo e accetta solo motori 0-23, posizioni 10-190. Errori di trasporto invalidano la sessione; la UI accorpa gli aggiornamenti e sospende gli slider durante il download.
- Il titolo di otto caratteri viene passato intero al protocollo; non viene più usato il suffisso dopo il settimo carattere. Il formato oggetto e l'immagine a sette caratteri restano invariati.
- La transizione immediata del baud dopo l'ultimo blocco non è misurata. Nel nuovo collaudo, chiusura e riapertura a 9600 baud dopo MUSIC funzionano su COM1. Il comando di download consuma e chiude la sessione sia sul successo sia su errore di trasferimento; non imposta ciecamente il PC a 9600 mantenendo una connessione apparentemente valida. Un errore di compilazione precedente al trasferimento lascia la sessione aperta.
- Test sul robot: `cargo test --lib real_info -- --ignored --nocapture` (sola lettura, ora fallisce su errore), `real_beep` (scrive BEEP+END), `real_music` (scrive MUSIC "CDE"+END). Richiedono `ROBO_PORT` esplicita; i due test di scrittura richiedono anche `ROBO_ALLOW_WRITE=YES` dopo il consenso. Non eseguire tutti gli ignorati in automatico. Il `BEEP` non si sente sul robot, anche con l'IDE originale produce gli stessi byte; `MUSIC` si sente nelle prove storiche.

## Simulatore e verifica

- La verifica dell'editor usa il compilatore del download; il simulatore viene validato con lo stesso compilatore e usa il suo lexer e i suoi gruppi.
- Aritmetica sequenziale, precedenza tra classi, INTEGER unsigned, bit, NOT, PRINT/FORMAT, MOVEPOS (anche in IF/THEN), `MOVE24`, NEXT senza variabile e limite di 192 byte sono coperti da regressioni. Valori MOVE omessi: 00 nel bytecode, posizione precedente nella simulazione. Tutti i 274 campioni compilabili vengono anche controllati dal parser del simulatore.
- Sono riprodotti flusso di controllo, posizioni servo, abilitazione MOTOR/MOTOROFF, SPEED, output PRINT/CLS e tempo DELAY. I movimenti sono istantanei, non un modello fisico. Tempi di movimento, aritmetica di runtime del firmware e interazioni sensori non sono stati nuovamente misurati sul robot.
- PTP, direzioni/configurazioni motori, LCD avanzato, WAIT, suono e configurazioni sensori vengono segnalati come effetti hardware non simulati. Operazioni non gestite, come STOP, accesso memoria e comunicazione, interrompono la simulazione.
- Le funzioni hardware non restituiscono più zero inventato: interrompono con diagnostica. MOTORIN legge la posizione simulata con un avviso, non il robot.
- Il manuale BREAK specifica che il comando non opera sul MR-C3000: non deve essere interpretato come uscita da FOR.
- ZERO, INIT, POS come dichiarazione, RUN e configurazione AIMOTOR non modificano lo stato fisico nella simulazione: producono avvisi. KEYIN non restituisce dati analogici inventati. LCD viene rappresentato come testo, non come modulo fisico.
- DEVICE/LIMIT sono gestiti dall'applicazione, non dall'OCX o dal firmware. MR-C3024/MRC3000 sono gli unici target; limiti per motori 0-23, valori 10-190, ultima dichiarazione per motore prevalente. Slider e backend live li applicano al sorgente corrente senza movimenti automatici nell'aggiornamento.
- Verifica e download rifiutano angoli espliciti fuori limite, SERVO dinamici non verificabili e operazioni indirette ACTION/FUNC/POKE/ROMPOKE/AIMOTOR INIT quando esistono LIMIT. Non si pretende protezione della posizione gia' presente, dell'attivazione motori o della calibrazione EEPROM; LIMIT non e' un fermo hardware.

## Nuovo collaudo e verifiche residue

La lettura `real_info` è stata eseguita su COM1, indicata dall'utente. Dopo consenso specifico è stato trasferito un singolo blocco con `MUSIC "CDE"` seguito da `END` (nome `MUSICTST`); il programma precedentemente salvato è stato sostituito. Una nuova lettura dopo la chiusura del trasferimento risponde a 9600 baud con gli stessi sei byte. Non sono stati inviati comandi di movimento; suono percepito e titolo visualizzato non sono stati confermati dall'utente. Prima di muovere servo concordare motore, posizione e condizioni di sicurezza; verificare eco e successiva riconnessione senza ritentativi automatici.

Lo smoke test della release Windows verifica avvio reale Tauri/WebView2, stato iniziale dei pulsanti, diagnostica via IPC, espressioni sequenziali e aggiornamento servo simulato, interruzione su STOP, indice help e rendering Markdown. Questo smoke test non apre la seriale.

## Non fatto
- Altri controller (MRC-2000, MA2K, MA3K): servono campioni con `Con_Type`/`Con_ID` diversi e la cattura del protocollo.
- Parità di tutti i possibili errori fuori dal corpus e dei testi originali: non garantita. Le righe dei 78 negativi esistenti sono confrontate con dati persistiti dell'OCX. MOVEPOS espanso e il rifiuto dei bit BYTE oltre 7 sono differenze intenzionali documentate.
- Nuovo collaudo servo, conferma fisica di suono/titolo e misura della transizione immediata del baud finale: ancora da eseguire con consenso ove necessario. Download MUSIC e riconnessione a 9600 sono già verificati.
