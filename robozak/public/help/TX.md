**TX       Trasmissione**

Trasmette un segnale RS232 tramite la porta TX. 2000

![](TX.files/image001.gif)

             -
Sintassi inglese : TX *[velocità porta], [dati]*

             -
Sintassi coreana : 송신 *[velocità porta], [dati]*

 

![](TX.files/image002.gif)

Trasmette dati in modalità RS232 tramite la porta 10 dei controller della serie MR-C2000. Per *[velocità porta]* si usa un numero o una costante da 1 a 4; il significato di *[velocità porta]* è il seguente.

 

|  |  |
| --- | --- |
| **Numero** | **Impostazione della porta** |
| 1 | 1200bps, 8Bit data, No parity, 1 Stop bit |
| 2 | 2400bps, 8Bit data, No parity, 1 Stop bit |
| 3 | 2400bps, 8Bit data, No parity, 1 Stop bit |
| 4 | 4800bps, 8Bit data, No parity, 1 Stop bit |

 

 *[dati]* è il valore dei dati inviati tramite la porta di trasmissione. Per *[dati]* si possono usare numeri, costanti, variabili, ecc. Per trasmettere il carattere "A" occorre inviare il codice ASCII corrispondente a "A", cosa che si può fare semplicemente così:

 

DIM I AS BYTE

 

I = "A"

TX 4, I

 

 Cioè, assegnando una stringa a una variabile, alla variabile viene assegnato il codice ASCII del primo carattere della stringa.

*Nei controller della serie MR-C3000, per trasmettere segnali RS232 si usa il comando ETX (데이타송신) al posto del comando TX.*

 

![](TX.files/image003.gif)

 Il seguente è un esempio di programma che trasmette continuamente il valore del tasto della porta byte 0 tramite il terminale RS232 collegato all'esterno.

 

             DIM 
A  AS  BYTE

 

 Main:

             A
= BYTEIN(0)

             TX 
4, A

             GOTO
Main
