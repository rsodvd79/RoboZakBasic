**KEYIN()      Tastiera()**

Legge l'ingresso di più tasti (tastierino analogico). 3000

![](KEYIN.files/image001.gif)

             -
Sintassi inglese : KEYIN(*[numero porta analogica], [numero tasti]*)

             -
Sintassi coreana : 키보드(*[numero porta analogica], [numero tasti]*)

 

![](KEYIN.files/image002.gif)

 Tramite la porta di conversione AD (porta analogica) di un controller serie MR-C3000, legge il valore del pulsante premuto, ricavato da un circuito analogico partitore di tensione che comprende fino a 16 pulsanti.

 *[numero porta analogica]* può essere un numero, una costante o una variabile di tipo byte da 0 a 7, e *[numero tasti]* un numero, una costante o una variabile di tipo byte da 1 a 16.

 Il valore del pulsante letto va da 0 a 16: 0 indica che nessun tasto è premuto, da 1 a 16 indica il numero del pulsante premuto.

 

![](KEYIN.files/image003.gif)

DIM K AS BYTE

 

K = KEYIN(0,
16)   *‘Assegna a K il valore del tastierino analogico composto da 16 tasti (pulsanti) collegato alla porta AD0.*
