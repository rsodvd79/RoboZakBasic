**ROMPEEK()           롬읽기()**

Legge i dati dalla EEPROM esterna.

![](ROMPEEK.files/image001.gif)

             -
Sintassi inglese : ROMPEEK (*[area ROM]*)

             -
Sintassi coreana : 롬읽기 (*[area ROM]*)

 

![](ROMPEEK.files/image002.gif)

Il mini robot controller utilizza una EEPROM per memorizzare il programma utente e per scopi particolari. La funzione ROMPEEK e il comando ROMPOKE, che controllano la memoria esterna, possono essere usati per salvare in modo permanente i dati desiderati. Tuttavia, usando un'area già riservata si possono causare errori fatali.  *[area ROM]* può essere un numero, una costante o una variabile.
