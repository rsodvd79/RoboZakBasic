# Istruzioni per Copilot su Windows (RoboZakBasic)

Contesto: riscrittura in Rust/Tauri v2 di **roboBASIC v2.72** (IDE VB6 per Robonova/MRC-3000). Rispondi in italiano, in modo conciso.
Leggi prima `re/NOTES.md` (stato del reverse engineering) e `README`/codice in `robozak/`.

## Stato
- Compilatore MR-C3024 in `compiler.rs`: 281 campioni positivi e 87 negativi verificati contro l'originale; inventario e righe degli errori in `re\samples\expectations.json`.
- Forme del manuale v2.10 aggiunte: PULSE, ZERO, INIT, POS/MOVEPOS, RUN, KEYIN a due argomenti, NOT(operando), basi numeriche, %, bit, PRINT/FORMAT, modalita' e gruppi AIMOTOR, baud seriali mancanti. MOVEPOS e' espanso in MOVE verificati, non in opcode sconosciuti.
- `directives.rs` valida DEVICE e LIMIT per MR-C3024. I limiti sono locali agli angoli comandati, non protezioni firmware; con LIMIT si rifiutano movimenti dinamici e operazioni indirette non verificabili. Aggiornare gli slider non invia comandi.
- MOVE/MOVE24 rifiutano angoli espliciti fuori 10-190; MUSIC supporta spazi, durata persistente, O34, M/H/L e parentesi quadre. GOTO AUTO usa il bytecode originale, viene rifiutato con LIMIT e interrompe il simulatore senza inventare il template.
- Operandi hardware statici e limite prudenziale di 32 KB (immagine con header) sono condivisi da verifica, simulazione e download, anche senza LIMIT. Il simulatore controlla i valori dinamici; il compilatore evita il wrap degli indirizzi/lunghezze a 16 bit. Questi controlli sono intenzionalmente piu' restrittivi dell'OCX.
- Verifica editor e simulazione usano il dialetto del compilatore. Il simulatore (`basic.rs`) ha movimenti istantanei, segnala effetti hardware non riprodotti e non inventa letture dei sensori.
- Protocollo di download e controllo live in `download.rs`, sessione seriale e diagnostica Tauri in `lib.rs`. Eco servo validato; errori di trasporto e tentativi di download chiudono la sessione.
- Gli originali `rbv2.ocx` e `rbv2.exe` sono **VB6 codice nativo x86**, non p-code. Il metodo efficace è l'OCX come black box più catture seriali, non una nuova decompilazione generale.
- Il nuovo collaudo autorizzato su COM1 passa: informazioni (`02 02 07 40 BE 05`), download MUSIC + END e riconnessione a 9600 baud. Nessun movimento è stato inviato; il nuovo collaudo servo richiede consenso separato.

## Setup
1. `gh repo clone rsodvd79/RoboZakBasic`
2. Installa Rust (rustup), Node LTS, WebView2 (già in Windows 11) e Visual Studio Build Tools (C++).
3. Dalla radice: `npm --prefix .\robozak install`, poi `npm --prefix .\robozak run tauri dev`.
4. Test Rust: `cargo test --manifest-path .\robozak\src-tauri\Cargo.toml --lib`. Test frontend: `npm --prefix .\robozak test`. Build: `npm --prefix .\robozak run tauri build -- --no-bundle`.
5. Installa roboBASIC dall'MSI `roboBASIC v2.72.msi` (non è nel repo: copialo a mano). Estrai anche una copia dei file in `extracted/` (gitignored) con `msiexec /a "roboBASIC v2.72.msi" /qb TARGETDIR=C:\rb_extract`.

## Verifica e limiti
- Preservare tutti i campioni del compilatore e aggiornare le aspettative con l'OCX, non con il compilatore reimplementato. Vedi `README.md` per il comando di registrazione.
- Il supporto resta MR-C3024. MRC-2000, MA2K e MA3K richiedono campioni e protocolli propri e sono fuori ambito.
- Dopo `EC` il trasferimento passa da 9600 a 115200 baud. La transizione immediata dopo l'ultimo blocco non è stata misurata; chiusura e riapertura a 9600 sono verificate su COM1. Non ripristinare soltanto il baud del PC presumendo quello del robot.
- Per test reali scegliere esplicitamente `ROBO_PORT`; non eseguire quelli ignorati in automatico. Prima di sostituire programmi o muovere servo chiedere consenso specifico. Le scritture `real_beep` e `real_music` richiedono anche `ROBO_ALLOW_WRITE=YES`.
- Aggiornare le note distinguendo prove automatiche, storiche e nuove prove reali. Commit e push solo quando richiesti.
- `cargo fmt --check` passa; Clippy rigoroso ha ancora due avvisi preesistenti nel compilatore, documentati nel README.

## Note tecniche
- File `.cmd`: CSV `ENGLISH,korean` in CP949 (`*e.cmd` solo inglese).
- Gruppi motori: G6A..G6D = motori 0-23 (6 ciascuno), G8x = 8, G12/16/24/32 partono da 0. Angolo servo ~10-190 (default 100).
- MR-C2000: 30 byte nei manuali. Per il compilatore MR-C3024 verificato, variabili a `0x40..0xFF`: 192 byte utilizzabili; oltre il limite l'originale produce un programma vuoto senza errore.
- Aritmetica/bit MR-C3024 sequenziali: `1 + 2 * 3` vale 9; i confronti precedono le loro combinazioni logiche. INTEGER unsigned 0-65535. NOT richiede `NOT(operando)`; parentesi di raggruppamento, array e STEP restano esclusi. Bit BYTE oltre 7 sono rifiutati anche se l'OCX li accetta.
- Tauri converte gli argomenti snake_case Rust in camelCase in JS.
- Non committare gli originali Microsoft/Hanool (`extracted/`, `chm/`, MSI): sono in `.gitignore`.
