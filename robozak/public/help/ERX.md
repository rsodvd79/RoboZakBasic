**ERX           데이타수신**

Riceve un segnale RS232 tramite la porta ERX. 3000

![](ERX.files/image001.gif)

             -
Sintassi inglese : ERX *[velocità porta], [variabile di ricezione], [etichetta gestione errore]*

             -
Sintassi coreana : 데이타수신 *[velocità porta], [variabile di ricezione], [etichetta gestione errore]*

 

![](ERX.files/image002.gif)

Riceve dati in formato RS232 tramite la porta ERX del controller della serie MR-C3000. Il significato di *[velocità porta]* è il seguente.

 

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

 

 *[variabile di ricezione]* è la variabile in cui salvare i dati ricevuti; deve essere obbligatoriamente una variabile di tipo byte già dichiarata.

 *[etichetta gestione errore]* è l'etichetta da cui inizia la parte da eseguire se non sono stati ricevuti dati (se il buffer di comunicazione è vuoto). Un programma che attende finché non vengono ricevuti dati tramite la porta RS232 si può scrivere così:

 

Retry:

             ERX
 9600,  A,  Retry

 

 *Nei controller della serie MR-C2000, per ricevere un segnale RS232 si usa il comando RX al posto del comando ERX.*
