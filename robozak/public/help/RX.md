**RX       수신**

Riceve un segnale RS232 tramite la porta RX. 2000

![](RX.files/image001.gif)

             -
Sintassi inglese : RX *[velocità porta], [variabile di ricezione], [etichetta gestione errore di ricezione]*

             -
Sintassi coreana : 수신 *[velocità porta], [variabile di ricezione], [etichetta gestione errore di ricezione]*

 

![](RX.files/image002.gif)

Riceve dati in modalità RS232 tramite la porta 9 dei controller della serie MR-C2000. *[velocità porta]* è un numero o una costante da 1 a 4, con il seguente significato.

 

|  |  |
| --- | --- |
| **Numero** | **Impostazione porta** |
| 1 | 1200bps, 8Bit data, No parity, 1 Stop bit |
| 2 | 2400bps, 8Bit data, No parity, 1 Stop bit |
| 3 | 2400bps, 8Bit data, No parity, 1 Stop bit |
| 4 | 4800bps, 8Bit data, No parity, 1 Stop bit |

 

 *[variabile di ricezione]* è la variabile in cui salvare i dati ricevuti; deve essere una variabile di tipo byte già dichiarata.

 *[etichetta gestione errore di ricezione]* è l'etichetta da cui inizia la parte di codice eseguita se non sono stati ricevuti dati (cioè il buffer di comunicazione è vuoto). Un programma che attende finché non arrivano dati dalla porta RS232 si può scrivere così:

 

Retry:

             RX
 4,  A,  Retry

 *Nei controller della serie MR-C3000, per ricevere segnali RS232 si usa il comando ERX(데이타수신) al posto del comando RX(수신).*

 

![](RX.files/image003.gif)

 Il seguente esempio è un programma che, se attraverso il terminale RS232 collegato all'esterno arriva il valore del codice ASCII &h80 (esadecimale), accende il LED della porta 0, altrimenti lo spegne.

 

             DIM 
A  AS  BYTE

 

Retry:   RX 4, A, Retry

             IF
A = &h80 THEN

                           OUT
0, 0

             ELSE

                           OUT
0, 1

             ENDIF

             GOTO
Retry
