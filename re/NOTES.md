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

## Prossimi passi
1. Catturare il traffico seriale di roboBASIC (download di un `.bas` minimo e controllo servo live).
2. Confrontare i byte con il programma sorgente per dedurre opcode e frame.
3. Implementare `lang-compiler` e l'invio live dei servo in `serial_write`.
