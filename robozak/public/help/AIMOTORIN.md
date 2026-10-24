**AIMOTORIN()   Ingresso motore AI()**

Legge il valore corrente del motore AI. 3000

![](AIMOTORIN.files/image001.gif)

             -
Sintassi inglese : AIMOTORIN (*[numero motore]*)

             -
Sintassi coreana : AI모터입력 (*[numero motore]*)

 

![](AIMOTORIN.files/image002.gif)

Acquisisce il valore della posizione corrente del motore AI collegato a un controller serie MR-C3000, permettendo di controllarlo facilmente da programma.

*[numero motore]* può essere un numero o una costante da 0 a 30; se è collegato un servomotore, si legge l'angolo del motore (10-190), mentre se non è collegato alcun servomotore si legge il valore 0.

 

![](AIMOTORIN.files/image003.gif)

DIM  AI5  AS  BYTE

 

AIMOTOR INIT

AIMOTOR SETON

AIMOTOR 5                                *‘Abilita l'uso
del motore AI numero 5*

  AI5 =
AIMOTORIN(5)                   *‘Salva il valore del
motore AI numero 5 nella variabile AI5.*
