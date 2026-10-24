**GOSUB ... RETURN     Chiamata
... Ritorno**

Chiama ed esegue una subroutine, poi ritorna.

![](RETURN.files/image001.gif)

             -
Sintassi inglese :        GOSUB *[etichetta di riga]*

                                          
..............

        [etichetta di riga]:  
..............

                                          
.............

                                        RETURN

 

             -
Sintassi coreana :        호출 *[etichetta di riga]*

                                          
..............

         [etichetta di riga]:  
..............

                                          
.............

                                        복귀

 

![](RETURN.files/image002.gif)

Una subroutine (Sub Routine) descrive una sola volta le parti di programma usate di frequente e, a ogni utilizzo, viene chiamata per eseguire il salto e il ritorno. Le subroutine riducono le dimensioni del programma e permettono di scrivere programmi più efficienti.

 All'interno di una subroutine si possono chiamare altre subroutine, e *il numero di chiamate annidate consentite è 4 nella serie MR-C2000 e 5 nella serie MR-C3000.*
Superare il limite del controller provoca gravi errori interni, quindi evitare chiamate annidate complesse.

 

![](RETURN.files/image003.gif)

DIM LED\_PORT AS INTEGER

 

LED\_PORT = 1

START:
.............

.............

GOSUB LED\_TOGGLE

...............

GOTO START

END

 

LED\_TOGGLE:

TOGGLE LED\_PORT

RETURN
