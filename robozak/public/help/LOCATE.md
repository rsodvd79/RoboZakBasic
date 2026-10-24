**LOCATE          화면위치**

Specifica la posizione del testo da visualizzare sul modulo LCD.

![](LOCATE.files/image001.gif)

             -
Sintassi inglese : LOCATE *[coordinata x], [coordinata y]*

             -
Sintassi coreana : 화면위치 *[coordinata x], [coordinata y]*

 

![](LOCATE.files/image002.gif)

Il comando LOCATE(화면위치) imposta la posizione, tramite le coordinate x e y, in cui visualizzare un carattere sul modulo LCD.

 

 Le coordinate di un modulo LCD a 16x2 righe sono definite come nella figura sopra; per gli altri moduli LCD le coordinate sono definite allo stesso modo. Le coordinate x, y possono essere numeri, costanti o variabili; si noti che *partono sempre da 0*.

|  |
| --- |
|  |
|  |  |

![](LOCATE.files/image004.gif)

LOCATE 0, 0       *'Sposta il cursore del modulo LCD alla posizione iniziale.*

LOCATE 4, 1       *'Sposta il cursore del modulo LCD alle coordinate (4, 1).*
