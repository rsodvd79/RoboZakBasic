# RoboZakBasic

Riscrittura moderna (Rust + Tauri v2) di **roboBASIC v2.72**, l'IDE VB6 di HITEC per programmare i robot Robonova con controller **MR-C3024** (MRC-3000).

## Funzioni

- **Editor** con evidenziazione delle keyword e aiuto sui comandi.
- **Compilatore** BASIC → codice oggetto del controller, identico byte per byte all'originale su 242 campioni (anche per gli errori e per il limite di 192 byte di variabili).
- **Simulatore** per eseguire il programma senza robot.
- **Download** del programma sul robot via seriale (COM, 9600 baud, poi 115200 dopo `EC`).
- **Controllo live dei servo**: gli slider sul disegno del robot muovono i motori (`E6 <motore> <pos>`).

Verificato su robot reale MR-C3024 (download ed esecuzione di un programma con `MUSIC`, movimento dei servo). Altri controller (MRC-2000, MA2K, MA3K) non sono supportati.

## Struttura

| Cartella | Contenuto |
|----------|-----------|
| `robozak/` | App Tauri: frontend TypeScript/Vite in `src/`, backend Rust in `src-tauri/src/` (`compiler.rs`, `download.rs`, `basic.rs`, `lib.rs`) |
| `re/` | Reverse engineering: `NOTES.md` (protocollo e formato), `samples/` (sorgenti `.bas` e output originale `.hex`, usati come oracolo nei test), `scripts/` (Frida, Ghidra, robot finto) |
| `msi/` | Installer originale roboBASIC v2.72 |

## Requisiti

- Node.js e npm
- Rust (toolchain stabile) e i prerequisiti di [Tauri v2](https://tauri.app/start/prerequisites/)
- Windows (porta seriale reale; lo sviluppo del reverse engineering usa anche l'OCX originale a 32 bit)

## Uso

```powershell
cd robozak
npm install
npm run tauri dev      # avvio in sviluppo
npm run tauri build    # pacchetto di rilascio
```

Nell'app: scegli la porta, **Connetti**, poi **Scarica sul robot** oppure muovi gli slider.

## Test

```powershell
cd robozak\src-tauri
cargo test --lib
```

Il test `matches_original_compiler` confronta il compilatore con i campioni in `re/samples`. Per rigenerare i campioni con l'originale (COM a 32 bit): `re\scripts\rbcases.ps1 -Filter <regex>`.

Test sul robot reale (ignorati di default):

```powershell
$env:ROBO_PORT="COM1"
cargo test real_info -- --ignored --nocapture   # sola lettura
cargo test real_beep -- --ignored --nocapture   # scrive un programma sul robot
```

## Protocollo

Il protocollo seriale e il formato del codice oggetto sono documentati in [`re/NOTES.md`](re/NOTES.md). Sono stati ricavati con Ghidra, Frida (`re/scripts/frida_serial.py`) e confronti sistematici con il compilatore originale usato come black box.

## Note legali

Progetto indipendente, non affiliato a HITEC. roboBASIC è software di HITEC; questo repository contiene solo una reimplementazione e note di interoperabilità.
