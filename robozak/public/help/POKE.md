**POKE   램쓰기**

Scrive dati nella memoria interna.

![](POKE.files/image001.gif)

             -
Sintassi inglese : POKE *[area RAM]*, *[dati]*

             -
Sintassi coreana : 램쓰기 *[area RAM]*, *[dati]*

 

![](POKE.files/image002.gif)

Il comando POKE si usa per scrivere dati nella memoria interna del mini robot controller. La funzione PEEK si limita a leggere i dati, mentre il comando POKE può scriverli direttamente, quindi richiede la massima cautela.

 *[area RAM]* può essere un numero, una costante o una variabile di tipo byte tra 0 e 255 per i controller della serie MR-C2000, e tra 0 e 65535 per i controller della serie MR-C3000. *[dati]* può essere un numero, una costante o una variabile (comprese le variabili intere).

 

![](POKE.files/image003.gif)

POKE &h40,
100               *‘Scrive il valore 100 all'indirizzo 40 (esadecimale) della RAM.*
