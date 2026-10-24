**PEEK()      Lettura RAM()**

Legge il contenuto della memoria interna.

![](PEEK.files/image001.gif)

             -
Sintassi inglese : PEEK (*[area RAM]*)

             -
Sintassi coreana : 램읽기 (*[area RAM]*)

 

![](PEEK.files/image002.gif)

 La funzione PEEK legge i dati nella memoria interna, ma non si può usare senza conoscere con precisione la struttura della memoria interna, quindi non va manipolata alla leggera.

 L'area utilizzabile per *[area RAM]* dipende dal controller.

 

Controller della serie MR-C2000)

Si possono usare un numero compreso tra 0 e 255, una costante o una variabile di tipo byte.

 

Controller della serie MR-C3000)

Si possono usare un numero compreso tra 0 e 255, una costante o una variabile di tipo byte.

 

![](PEEK.files/image003.gif)

DIM A AS BYTE

 

A = PEEK(43)                    *‘Legge nella variabile A il valore dell'indirizzo RAM 43.*
