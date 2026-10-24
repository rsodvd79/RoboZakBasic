**GOTO        가기**

Salta incondizionatamente alla posizione indicata.

![](GOTO.files/image001.gif)

             -
Sintassi inglese : GOTO *[etichetta di riga]*

             -
Sintassi coreana : 가기 *[etichetta di riga]*

 

![](GOTO.files/image002.gif)

Il comando GOTO (가기) salta incondizionatamente alla riga indicata dall'etichetta; l'etichetta di riga deve essere dichiarata obbligatoriamente all'interno del programma. Un uso eccessivo dell'istruzione GOTO rende il programma difficile da comprendere, quindi usarla entro limiti non complessi e, per quanto possibile, preferire le istruzioni strutturate di controllo del flusso (IF...THEN, FOR...NEXT).

 

![](GOTO.files/image003.gif)

DIM I AS INTEGER

DIM J AS BYTE

 

I = 7

IF I = 6 THEN GOTO L1

  ................

L1:  J = 1

OUT I, J
