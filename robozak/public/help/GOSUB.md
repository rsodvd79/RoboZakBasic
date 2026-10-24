**GOSUB ... RETURN     호출
... 복귀**

Chiama ed esegue una subroutine, poi ritorna.

![](GOSUB.files/image001.gif)

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

![](GOSUB.files/image002.gif)

Una subroutine (Sub Routine) è una parte di programma usata di frequente, descritta una sola volta e chiamata ogni volta che serve, eseguendo salto e ritorno. Le subroutine riducono la dimensione del programma e permettono di scrivere programmi efficienti.

 All'interno di una subroutine è possibile chiamare altre subroutine; *il numero di chiamate annidate consentite è 4 nella serie MR-C2000 e 5 nella serie MR-C3000.* Chiamate annidate oltre il limite del controller causano gravi errori interni, quindi occorre evitare chiamate annidate complesse.

 

![](GOSUB.files/image003.gif)

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
