**MINIIN       미니입력**

Riceve un segnale minibus tramite la porta minibus. 2000

![](MINIIN.files/image001.gif)

             -
Sintassi inglese : MINIIN

             -
Sintassi coreana : 미니입력

 

![](MINIIN.files/image002.gif)

Riceve dati in formato minibus tramite la porta minibus (porta 6) del controller della serie MR-C2000. Se non sono stati ricevuti dati, viene restituito il valore 0.

 Un programma che attende finché non arrivano dati tramite il minibus si può scrivere come segue.

 

DIM A AS BYTE

 

Retry:

A = MINIIN

IF A = 0 THEN GOTO Retry
