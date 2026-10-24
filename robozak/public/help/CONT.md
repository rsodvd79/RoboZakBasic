**CONT        Luminosità caratteri**

Regola la luminosità dei caratteri visualizzati sul modulo LCD.

 

![](CONT.files/image001.gif)

             -
Sintassi inglese : CONT *[valore di luminosità]*

             -
Sintassi coreana : 글자밝기 *[valore di luminosità]*

 

![](CONT.files/image002.gif)

Il modulo LCD 16x2 righe usato con il controller del mini robot è di tipo retroilluminato (Back Light): in uso, si accende una luce dietro il display a cristalli liquidi. I caratteri visualizzati sul modulo LCD sono neri, e il comando CONT (글자밝기) regola l'intensità di questo nero. Per [valore di luminosità] si possono usare numeri, costanti e variabili. Più il [valore di luminosità] è piccolo, più il nero si scurisce meno, quindi sul modulo LCD i caratteri appaiono sempre più sbiaditi. Il valore iniziale di luminosità dei caratteri del modulo LCD è 7. Il modulo LCD coreano non ha la regolazione della luminosità ma ha la funzione di inversione. Per invertire, impostare il valore di luminosità a 0.

 

![](CONT.files/image003.gif)

 

LCDINIT

 

CONT 10

PRINT
"miniROBOT"
