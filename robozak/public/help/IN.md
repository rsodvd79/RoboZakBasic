**IN()      Ingresso()**

Legge il valore di un segnale digitale da una porta.

![](IN.files/image001.gif)

             -
Sintassi inglese : IN(*[numero porta]*)

             -
Sintassi coreana : 입력(*[numero porta]*)

 

![](IN.files/image002.gif)

Riceve un segnale tramite una porta del mini controller robot e ne memorizza il valore in una variabile. Il valore ricevuto dalla porta è 0 oppure 1; la variabile usata può essere di tipo byte o intero e, in tal caso, è valido solo il valore dell'ultimo bit (bit 0). Il metodo più efficiente è ricevere il valore specificando un bit di una variabile di tipo byte. Se si legge il valore del segnale mentre alla porta non è collegato alcun dispositivo, si possono ottenere valori completamente diversi: fare attenzione.

 

![](IN.files/image003.gif)

DIM A AS BYTE

 A =
IN(0)     *'Riceve un segnale (interruttore o sensore) sulla porta 0 e lo inserisce nella variabile A.*
