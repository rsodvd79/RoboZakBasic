**ETX           Trasmissione dati**

Trasmette un segnale RS232 tramite la porta ETX. 3000

![](ETX.files/image001.gif)

             -
Sintassi inglese : ETX *[velocità porta], [dati]*

             -
Sintassi coreana : 데이타송신 *[velocità porta], [dati]*

![](ETX.files/image002.gif)

Trasmette dati in formato RS232 tramite la porta ETX di un controller serie MR-C3000. Il significato di *[velocità porta]* è il seguente.

 

|  |  |
| --- | --- |
| **Numero** | **Impostazione porta** |
| 2400 | 2400bps, 8Bit data, No parity, 1 Stop bit |
| 4800 | 4800bps, 8Bit data, No parity, 1 Stop bit |
| 9600 | 9600bps, 8Bit data, No parity, 1 Stop bit |
| 14400 | 14400bps, 8Bit data, No parity, 1 Stop bit |
| 19200 | 19200bps, 8Bit data, No parity, 1 Stop bit |
| 28800 | 28800bps, 8Bit data, No parity, 1 Stop bit |
| 38400 | 38400bps, 8Bit data, No parity, 1 Stop bit |
| 57600 | 57600bps, 8Bit data, No parity, 1 Stop bit |
| 76800 | 76800bps, 8Bit data, No parity, 1 Stop bit |
| 115200 | 115200bps, 8Bit data, No parity, 1 Stop bit |
| 230400 | 230400bps, 8Bit data, No parity, 1 Stop bit |

 

*[dati]* è il valore dei dati inviato tramite la porta di trasmissione. Per *[dati]* si possono usare numeri, costanti, variabili, ecc. Per trasmettere il carattere "A" occorre inviare il codice ASCII corrispondente a "A", cosa che si può fare semplicemente così:

 

DIM I AS BYTE

 

I = "A"

ETX  9600, I

 

 Cioè, quando si assegna una stringa a una variabile, alla variabile viene assegnato il codice ASCII del primo carattere della stringa.

 *Nei controller serie MR-C2000, per trasmettere segnali RS232 si usa il comando TX (송신) al posto del comando ETX (데이타송신).*
