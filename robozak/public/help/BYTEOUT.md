**BYTEOUT   바이트출력**

Invia un segnale alle porte a livello di byte.

![](BYTEOUT.files/image001.gif)

             -
Sintassi inglese : BYTEOUT *[numero porta byte]*, *[valore di uscita]*

             -
Sintassi coreana : 바이트출력 *[numero porta byte]*, *[valore di uscita]*

 

![](BYTEOUT.files/image002.gif)

Invia un valore alla porta a livello di byte del mini robot controller. *[numero porta byte]* può essere un numero, una costante o una variabile; *[valore di uscita]* è un numero tra 0 e 255, una costante o una variabile di tipo byte.

 

![](BYTEOUT.files/image003.gif)

BYTEOUT
0, 255*'Invia il segnale 1 (5V) a tutte le porte da 0 a 7.*

BYTEOUT
0, &h10101010

*'Segnale 1 (5V) alle porte 1,3,5,7*

*‘**Invia il segnale 0 (0V) alle porte 0,2,4,6.*
