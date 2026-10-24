**MOTORIN()      모터입력()**

Legge il valore corrente di un servomotore. 3000

![](MOTORIN.files/image001.gif)

             -
Sintassi inglese : MOTORIN (*[numero motore]*)

             -
Sintassi coreana : 모터입력 (*[numero motore]*)

 

![](MOTORIN.files/image002.gif)

Acquisisce il valore della posizione corrente dei servomotori dedicati ai robot (serie HSR- di HITEC) collegati al controller della serie MR-C3000, permettendo di controllarli facilmente da programma.

*[numero motore]* può essere un numero o una costante da 0 a 31; se il servomotore è collegato, viene letto l'angolo del motore compreso tra 10 e 190, mentre se il servomotore non è collegato viene letto il valore 0.

 

![](MOTORIN.files/image003.gif)

DIM  S0  AS  BYTE

 

MOTOR 0                                   *‘Usa il servomotore 0.*

  S0 =
MOTORIN(0)                       *‘Memorizza il valore del servomotore 0 nella variabile S0.*
