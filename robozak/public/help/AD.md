**AD()          변환()**

Converte in digitale il segnale analogico proveniente dalla porta di conversione AD. 3000

![](AD.files/image001.gif)

             -
Sintassi inglese : AD (*[porta AD]*)

             -
Sintassi coreana : 변환 (*[porta AD]*)

![](AD.files/image002.gif)

I controller della serie MR-C3000 hanno in totale 8 porte di conversione AD, dalla 0 alla 7 (porte di ingresso/uscita digitali dalla 32 alla 39), che convertono in digitale e leggono i segnali analogici provenienti da sensori o dispositivi esterni. In *[porta AD]* si usa un numero da 0 a 7, una costante o una variabile di tipo byte o intero.

 

![](AD.files/image003.gif)

 Il seguente esempio riceve un segnale analogico dalla porta AD 1 e ne visualizza il valore sul modulo LCD.

 

DIM
a AS BYTE                *'Dichiara una variabile di tipo byte chiamata a.*

LCDINIT              *'Inizializza l'uso del modulo LCD.*

CLS                                  *'Cancella tutto lo schermo LCD.*

CSOFF                             *'Nasconde il cursore.*

MAIN:                                          *'Dichiara l'etichetta MAIN.*

           a = AD(1)                          *'Salva nella variabile a il valore letto dalla porta AD 1.*

           LOCATE 5,0                     *'Posiziona il cursore in 5,0.*

           PRINT FORMAT(a,DEC,2) *'Visualizza sul modulo LCD il valore a letto*

*‘**in decimale con 2 cifre.*

GOTO MAIN                     *'Salta all'etichetta MAIN.*
