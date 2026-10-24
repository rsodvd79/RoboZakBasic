**DELAY       Ritardo**

Ritarda l'esecuzione del programma per il tempo specificato.

![](DELAY.files/image001.gif)

 

             -
Sintassi inglese : DELAY *[tempo di ritardo]*

             -
Sintassi coreana : 지연 *[tempo di ritardo]*

 

![](DELAY.files/image002.gif)

 

Serve a ritardare l'esecuzione del programma per il tempo specificato. Nei controller serie MR-C2000 l'intervallo di tempo è 10ms, nei controller serie MR-C3000 è 1ms.

Per [tempo di ritardo] si possono usare numeri, costanti, variabili, ecc.

 

![](DELAY.files/image003.gif)

 

Esempio per controller serie MR-C2000)

 

DELAY 10  *'Ritarda di 100ms. (intervallo di tempo 10ms \* 10 = 100ms =
0,1 secondi)*

 

Esempio per controller serie MR-C3000)

 

DELAY 500   *'Ritarda di 500ms. (intervallo di tempo 1ms \* 500 = 500ms =
0,5 secondi)*
