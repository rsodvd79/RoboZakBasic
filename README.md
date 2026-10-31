# RoboZakBasic

Riscrittura moderna (Rust + Tauri v2) di **roboBASIC v2.72**, l'IDE VB6 di HITEC per programmare i robot Robonova con controller **MR-C3024** (MRC-3000).

## Funzioni

- **Interfaccia** scura con pannelli adattivi, azioni distinte per verifica/simulazione/download e stato della connessione sempre leggibile.
- **Editor** con evidenziazione delle keyword, numeri di riga, posizione del cursore e guida comandi ricercabile.
- **Compilatore** BASIC → codice oggetto del controller: 281 campioni identici byte per byte all'originale, più 87 sorgenti rifiutati con la stessa riga d'errore e verifica del limite di 192 byte di variabili.
- **Verifica** nell'editor tramite lo stesso compilatore del download, con diagnostica in italiano.
- **Simulatore logico** del dialetto compilabile: aritmetica sequenziale, precedenza tra classi di operatori, INTEGER unsigned, bit, flusso di controllo, posizioni servo, output e tempo `DELAY`. Non riproduce tempi fisici, sensori o tutti gli effetti hardware.
- **Download** del programma sul robot via seriale (COM, 9600 baud, poi 115200 dopo `EC`).
- **Controllo live dei servo**: gli slider sul disegno del robot muovono i motori (`E6 <motore> <pos>`).

Il pannello robot offre **Diagramma** ed **Elenco servo**, con slider e valori numerici sincronizzati. Il badge distingue vista locale, simulata e controllo live; gli avvisi precisano quando gli slider muovono il robot reale. Console e guida hanno schede separate, navigabili anche da tastiera; gli errori riportano in primo piano la console.

La barra sorgenti offre **Nuovo**, **Carica**, **Salva** e **Salva con nome**, con finestre native per i file `.bas` e `.txt`. **Recenti** conserva localmente gli ultimi 20 file aperti o salvati con successo, anche dopo il riavvio: il piu' recente appare in cima, senza duplicati. Selezionare un file recente usa le stesse conferme del caricamento; file mancanti o non leggibili non sostituiscono il sorgente. Scorciatoie: `Ctrl+N`, `Ctrl+O`, `Ctrl+S`, `Ctrl+Shift+S`. Nome e percorso del file, asterisco e stato nell'editor distinguono le modifiche non salvate. Prima di cambiare sorgente o chiudere l'app si puo' salvare, scartare o annullare; annullamenti ed errori non cancellano il codice. Il caricamento accetta UTF-8 (anche con BOM), UTF-16 con BOM e vecchi file Windows-1252; il salvataggio usa UTF-8. Le operazioni sui file non inviano comandi al robot.

Il protocollo originale è stato verificato su robot reale MR-C3024 (download ed esecuzione di un programma con `MUSIC`, movimento dei servo). Il nuovo collaudo autorizzato su COM1 verifica informazioni `02 02 07 40 BE 05`, download di `MUSIC "CDE"` + `END` e successiva riconnessione a 9600 baud. Nessun movimento servo è stato inviato in questo collaudo; le nuove protezioni servo restano verificate con controller finto. Altri controller (MRC-2000, MA2K, MA3K) non sono supportati.

## Compatibilita con il manuale v2.10 / MR-C3024

Il confronto con `Robobasic_2-10.pdf` e l'OCX ha aggiunto queste forme, coperte da regressioni:

| Area | Forme supportate |
|---|---|
| Numeri e bit | `&H`, `&B`, `&O`, `%` come `MOD`, assegnazione ASCII, `A.0` / `A.BIT`; bit 0-7 per BYTE e 0-15 per INTEGER |
| Funzioni | `KEYIN(porta, tasti)` e `NOT(operando)`; la forma `NOT A` resta invalida |
| Motori e controllo | `PULSE`, `ZERO` (80-120), `MOVE` / `MOVE24` / `INIT` / `POS` (valori espliciti 10-190), `RUN`, `MOTOR ALLON`, `MOTOROFF ALLOFF` |
| Motori AI | `AIMOTOR SETON/SETOFF/INIT`, gruppi, `AIMOTOR ALLON` e `AIMOTOROFF ALLOFF` (31 canali del formato originale) |
| LCD | `PRINT "OK",65` stampa caratteri, non numeri decimali; `PRINT FORMAT(A,DEC/HEX/BIN[,cifre])`, con cifre predefinite, allineamento e troncamento |
| Seriale BASIC | `ERX` / `ETX`: 2400, 4800, 9600, 14400, 19200, 28800, 38400, 57600, 76800, 115200, 230400 baud |
| Pose | `MOVEPOS P1` richiama `P1: POS G6A,...`, anche in IF/THEN, con definizioni successive e piu' gruppi POS consecutivi |
| Template | `GOTO AUTO`, anche in IF/THEN, usa il richiamo firmware verificato sull'originale; AUTO e' un nome riservato, non un'etichetta utente |
| Musica | Spazi, virgola come pausa, `O34...`, `M`, `H` / `>`, `L` / `<`, `[ ... ]` anche annidati, alterazioni `#` / `+` / `$` / `-` e durata numerica persistente |

`MOVEPOS` viene **espanso in MOVE**: l'OCX attuale rifiuta la sintassi documentata, quindi non viene inventato un opcode nativo. Il risultato espanso e' confrontato con byte prodotti dall'originale; l'espansione aumenta la dimensione del programma. Si rifiutano inoltre bit oltre il tipo BYTE, anche se l'OCX li lascia accedere alla variabile successiva. Il corpus non certifica ogni sintassi o ogni errore del manuale, che include altri controller e refusi.

INTEGER usa 0-65535 e va in wrap su 16 bit. Aritmetica e operatori di bit hanno uguale precedenza e sono sequenziali (`1 + 2 * 3 = 9`); seguono confronti e combinazioni logiche di confronti. Gli angoli MOVE omessi mantengono la posizione precedente; uno zero scritto esplicitamente e' invece rifiutato, come dall'originale.

Esempio musicale del manuale: `MUSIC "O34GGAA GGE GGEED"`. `O` legge una sola cifra d'ottava (0-7), quindi `O34` significa ottava 3 e durata 4. La durata `n` vale 96/n e resta attiva per le note successive (0 equivale a 1); `[` e `]` dividono/moltiplicano la durata per 1,5. `L` scende di un'ottava, non e' un prefisso neutro: `L4` scende e imposta la durata 4. Le forme sono confrontate con l'OCX; i refusi sulle note puntate non sono interpretati come nuove sintassi.

`GOTO AUTO` passa al template gia' presente nel firmware Robonova-I e usa il primo indirizzo di variabile, come l'originale. Non installa un template nuovo. Il simulatore si interrompe esplicitamente al richiamo, senza eseguire i movimenti successivi del sorgente.

### Controlli del controller e dimensione programma

Verifica, simulazione e compilazione per download applicano gli stessi controlli anche senza LIMIT: servo fisici 0-23, posizioni 10-190, porte AD/KEYIN 0-7, tasti KEYIN 1-16, SONAR 0-11, SPEED 1-15, PWM/FPWM 0-2. Sono controllati anche operandi costanti nelle espressioni e nelle condizioni. Gli angoli espliciti di gruppi oltre il canale 23 sono rifiutati dall'app; i gruppi generici e gli alias globali del formato originale restano riconosciuti dal compilatore.

Questa validazione e' intenzionalmente piu' restrittiva dell'OCX, che accetta anche `SERVO 24,100`, `SERVO 0,255` e `AD(8)`. Senza LIMIT gli operandi variabili restano compilabili e vengono controllati quando eseguiti nel simulatore; non sono una garanzia dei valori che assumono sul firmware reale.

Il limite prudenziale MR-C3024 e' **32768 byte (32 KB) per l'immagine trasferita, header incluso**, secondo le specifiche nella cartella principale. Non e' una misura della capacita' effettivamente utilizzabile di ogni versione firmware. L'app rifiuta programmi oltre soglia prima della seriale; il compilatore impedisce inoltre il wrap di indirizzi/lunghezze a 16 bit e il download controlla la coerenza dell'header. Blocchi, padding e checksum del protocollo restano invariati.

### Direttive del sorgente

`'$DEVICE MR-C3024` (anche MRC3000) conferma il target; controller diversi sono rifiutati. `'$LIMIT 0,50,100` imposta i limiti locali del motore 0; sul MR-C3024 sono ammessi motori 0-23 e limiti 10-190. Le direttive sono globali al sorgente, con l'ultimo LIMIT per motore prevalente.

Gli slider applicano i limiti senza inviare movimenti durante l'aggiornamento; direttive invalide sospendono il controllo. Il backend verifica nuovamente il sorgente per ogni comando live. Verifica, simulazione e download rifiutano angoli espliciti fuori limite in SERVO/MOVE/MOVE24/POS/INIT. Con LIMIT, un motore SERVO dinamico o una posizione dinamica sul motore limitato non sono scaricabili: usare valori numerici o CONST. ACTION, FUNC, POKE, ROMPOKE, AIMOTOR INIT e GOTO AUTO sono rifiutati con LIMIT perche' possono modificare posizioni non verificabili.

La vista puo' limitare anche l'anteprima di una posizione iniziale simulata fuori intervallo, con un avviso: non cambia lo stato logico del simulatore o del robot. Risultati di verifica/simulazione relativi a un sorgente successivamente modificato non aggiornano la vista corrente.

**LIMIT non e' un limite firmware ne' una protezione meccanica.** Controlla gli angoli comandati dall'applicazione, non la posizione gia' presente, la calibrazione EEPROM, l'attivazione di un motore o altri programmi sul controller. Non sposta automaticamente un servo gia' fuori intervallo. Sensori, EEPROM, LCD fisico, suono, configurazione AI e ripresa RUN restano effetti hardware non simulati.

I PDF e le immagini hardware sono riferimenti, non una certificazione fisica del simulatore. Il protocollo HMI diretto del servo (19200 baud, 8N2) e il circuito Bluetooth non sostituiscono la seriale PC-controller (9600 baud, 8N1). `RoboNova IM Code.pdf` contiene opcode invertiti per gli shift e opcode errati per i salti a p. 9: valgono le codifiche verificate sull'OCX nelle note di interoperabilita'. INKEY, SOUND e PLAY sono indicati dal manuale per MR-C2000 e restano fuori ambito.

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

### Build automatiche

Il workflow GitHub Actions `Build Release` compila in Release a ogni push; puo' essere avviato anche manualmente dalla scheda **Actions**. Gli artifact Windows e macOS contengono ciascuno uno ZIP `RoboZakBasic.yyyy.MM.dd.HH.mm.zip`, con timestamp UTC: eseguibile `.exe` per Windows e bundle `.app` per macOS (Apple Silicon, runner `macos-latest`). L'app macOS non e' firmata ne' notarizzata; Gatekeeper puo' impedirne l'apertura.

Nell'app: scegli la porta, **Connetti** (identifica il controller a 9600 baud), poi **Scarica sul robot** oppure muovi gli slider. Il controllo live accetta motori 0-23 e posizioni 10-190; gli aggiornamenti sono serializzati e accorpati, e vengono sospesi durante il download.

Il download può sostituire ed eseguire il programma presente sul robot. Dopo ogni tentativo di trasferimento la sessione viene chiusa: il protocollo non fornisce una conferma finale e la transizione immediata del baud dopo l'ultimo blocco non è stata misurata. La chiusura e successiva riconnessione a 9600 baud sono verificate sul MR-C3024 di COM1. Riconnetti prima di usare nuovamente gli slider; se il robot non risponde a 9600 baud, non ritentare automaticamente scritture o movimenti. Anche un errore di eco/trasporto invalida la connessione. Un errore di compilazione, invece, non chiude la porta.

La simulazione mostra gli effetti hardware non riprodotti come avvisi. Si interrompe per operazioni non gestite (ad esempio `STOP`, comunicazione e accesso alla memoria); i sensori non restituiscono valori zero inventati. `MOTORIN` legge solo la posizione simulata. `BREAK` non interrompe i cicli sul MR-C3000. I manuali storici possono descrivere sintassi non accettate da questo compilatore.

## Test

```powershell
cd robozak\src-tauri
cargo test --lib
cargo fmt -- --check
```

Dal repository:

```powershell
npm --prefix .\robozak test
npm --prefix .\robozak run build
npm --prefix .\robozak run tauri build -- --no-bundle
```

Il test `matches_original_compiler` confronta il compilatore con tutti i campioni in `re\samples`: fallisce per campioni saltati, inventario incoerente, negativi accettati o righe d'errore diverse. `expectations.json` contiene gli esiti ricavati dall'OCX originale, non messaggi tradotti.

Per rigenerare campioni selezionati con l'originale (COM a 32 bit): `re\scripts\rbcases.ps1 -Filter <regex>`. Per registrare esiti e righe senza modificare gli `.hex`, dalla radice:

```powershell
.\re\scripts\rbcompile.ps1 .\re\samples -ExpectationsPath (Join-Path (Get-Location) 're\samples\expectations.json')
```

Il corpus verificato comprende 368 sorgenti, inclusi 16 nuovi casi per MOVE, MUSIC e AUTO; non garantisce la parità per ogni possibile errore. I controlli applicativi hardware e di capacita', piu' restrittivi dell'originale, e i valori dinamici del simulatore hanno regressioni separate. Clippy con `-D warnings` segnala ancora due avvisi preesistenti in `compiler.rs` (`unnecessary_get_then_check`, `collapsible_if`).

Test sul robot reale (ignorati di default):

```powershell
$env:ROBO_PORT="COM1" # scegliere esplicitamente la porta corretta
cargo test --lib real_info -- --ignored --nocapture # sola lettura, fallisce se il robot non risponde
# Solo dopo il consenso a sostituire/eseguire il programma:
$env:ROBO_ALLOW_WRITE="YES"
cargo test --lib real_beep -- --ignored --nocapture # scrive BEEP + END, nessun movimento
cargo test --lib real_music -- --ignored --nocapture # alternativa: scrive MUSIC "CDE" + END
Remove-Item Env:\ROBO_ALLOW_WRITE
```

Scegliere un solo test di scrittura in base al consenso, non entrambi automaticamente. Il `BEEP` può essere inaudibile sul MR-C3024 anche con l'originale. Il nuovo test MUSIC verifica il trasferimento, non certifica che il suono sia stato percepito. Il collaudo manuale dei servo richiede condizioni sicure e un consenso separato, non implicito nell'autorizzazione al download.

## Protocollo

Il protocollo seriale e il formato del codice oggetto sono documentati in [`re/NOTES.md`](re/NOTES.md). Sono stati ricavati con Ghidra, Frida (`re/scripts/frida_serial.py`) e confronti sistematici con il compilatore originale usato come black box.

## Note legali

Progetto indipendente, non affiliato a HITEC. roboBASIC è software di HITEC; questo repository contiene solo una reimplementazione e note di interoperabilità.
