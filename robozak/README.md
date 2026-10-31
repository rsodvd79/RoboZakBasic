# RoboZak: applicazione Windows

Frontend TypeScript/Vite e backend Rust/Tauri v2 per roboBASIC su MR-C3024. Requisiti, protocollo e limiti sono nel [README principale](https://github.com/rsodvd79/RoboZakBasic/blob/HEAD/README.md) e nelle [note di interoperabilità](https://github.com/rsodvd79/RoboZakBasic/blob/HEAD/re/NOTES.md).

Da questa cartella:

```powershell
npm install
npm run tauri dev
npm test
npm run build
npm run tauri build -- --no-bundle
cargo test --manifest-path .\src-tauri\Cargo.toml --lib
cargo fmt --manifest-path .\src-tauri\Cargo.toml -- --check
```

`src\main.ts` contiene editor, console, help e controllo degli slider; `src\main.test.mjs` verifica diagnostica e concorrenza con il test runner Node, senza robot o nuove dipendenze.

L'interfaccia ha un layout adattivo a tre pannelli, editor con numeri di riga e posizione del cursore, stato seriale visibile e schede console/guida accessibili da tastiera. La vista servo alterna diagramma ed elenco con valori numerici sincronizzati, distinguendo simulazione e controllo live. La finestra iniziale è 1280 × 820; su larghezze inferiori i pannelli si dispongono su più righe.

Backend in `src-tauri\src`: `compiler.rs` compila e valida, `basic.rs` simula il sottoinsieme logico, `directives.rs` gestisce DEVICE e LIMIT, `download.rs` gestisce il protocollo, `lib.rs` espone i comandi Tauri e invalida le sessioni seriali in errore.

Le forme aggiunte dal confronto con il manuale v2.10 sono elencate nel README principale. MOVEPOS e' tradotto in MOVE con bytecode verificato; INTEGER e' unsigned, i confronti precedono le combinazioni logiche e gli angoli MOVE omessi non modificano i servo. NOT usa parentesi e KEYIN richiede due argomenti.

LIMIT aggiorna i controlli senza movimenti automatici e viene ricontrollato nel backend. Verifica/download rifiutano angoli fuori limite e operazioni dinamiche/indirette non verificabili, incluso GOTO AUTO; non viene installato un limite firmware. Anche senza LIMIT si controllano gli operandi hardware statici e il limite prudenziale dell'immagine di 32 KB, header incluso. Il simulatore controlla gli operandi dinamici e interrompe GOTO AUTO senza simulare il template firmware. MOVE rifiuta angoli espliciti fuori 10-190; MUSIC accetta gli esempi documentati, spazi e parentesi quadre. Il corpus e' di 368 sorgenti (281 positivi e 87 negativi), senza certificare l'intero manuale o la fedelta' fisica del simulatore.

La simulazione usa il dialetto compilabile, ma non riproduce tutta la fisica o gli effetti hardware: avvisi e interruzioni sono visibili nella console. Dopo un download la connessione viene chiusa e richiede una nuova identificazione del controller. Non eseguire test reali, download o movimenti senza porta e consenso concordati; i test hardware restano ignorati di default.
