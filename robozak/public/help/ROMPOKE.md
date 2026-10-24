**ROMPOKE  Scrittura ROM**

Scrive dati nella EEPROM esterna.

![](ROMPOKE.files/image001.gif)

             -
Sintassi inglese : ROMPOKE *[area ROM], [dati]*

             -
Sintassi coreana : 롬쓰기 *[area ROM], [dati]*

 

![](ROMPOKE.files/image002.gif)

Il mini controller robot utilizza una EEPROM per memorizzare il programma utente e per scopi particolari. La funzione ROMPEEK e il comando ROMPOKE, che controllano la memoria esterna, possono essere usati per memorizzare dati che l'utente vuole conservare in modo permanente. Tuttavia, usando un'area già riservata possono verificarsi errori fatali. Per *[area ROM]* si possono usare numeri, costanti e variabili. Per *[dati]* si possono usare un numero compreso tra 0 e 255, una variabile di tipo byte o una costante.
