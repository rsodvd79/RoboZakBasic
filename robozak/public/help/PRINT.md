**PRINT  화면출력**

Visualizza del testo sul modulo LCD.

![](PRINT.files/image001.gif)

             -
Sintassi inglese : PRINT *“[stringa]” , [numero]/ “[stringa]”*, ....

             -
Sintassi coreana : 화면출력 *“[stringa]” , [numero]/ “[stringa]”*, ....

 

![](PRINT.files/image002.gif)

Il comando PRINT(화면출력) visualizza i caratteri nella posizione corrente del cursore del modulo LCD. *[stringa]* è delimitata da virgolette doppie ("). *[numero]* può essere un numero o una costante compresi tra 1 e 255 (lo 0 non è utilizzabile); sul modulo LCD viene visualizzato il carattere corrispondente al codice ASCII del numero. Con un modulo LCD coreano, in *[stringa]* si può usare l'hangul.

 

![](PRINT.files/image003.gif)

 

CLS

PRINT
"miniROBOT ", 126, " LCD"

|  |
| --- |
|  |
|  |  |

 

 

 

 

 

  

 Ecco l'insieme dei caratteri visualizzati quando si inviano caratteri con codice ASCII a un modulo LCD inglese 16x2 righe.

 I caratteri corrispondenti a ciascun codice possono variare a seconda del tipo di modulo LCD.

 

![](PRINT.files/image005.jpg)
