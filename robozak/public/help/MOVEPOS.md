**MOVEPOS        Posizione di movimento**

**POS                     Posizione dei motori**

Specifica ed esegue una determinata posizione dei motori. 3000

![](MOVEPOS.files/image001.gif)

             -
Sintassi inglese :  MOVEPOS *[etichetta di riga]*

                            
…………

                     
[etichetta di riga]: POS *[specifica gruppo], [angolo motore n],……*

 

             -
Sintassi coreana :  동작위치 *[etichetta di riga]*

                            
…………

                     
[etichetta di riga]: 모터위치 *[specifica gruppo], [angolo motore n],……*

 

![](MOVEPOS.files/image002.gif)

Nei controller serie MR-C3000, quando la posizione del robot ottenuta con MOVE (동작) deve essere richiamata da altre parti del programma, la si definisce con il comando POS (모터위치) e una *[etichetta di riga]*, e la si richiama dove serve con il comando MOVEPOS (동작위치). I comandi MOVEPOS e POS permettono di scrivere in modo conciso i programmi RoboBASIC con parti ripetute di frequente e ne facilitano la modifica.

 

![](MOVEPOS.files/image003.gif)

…………………………

MOVEPOS  POS01                        *‘**Esegue la parte con il comando POS*

*‘**alla posizione dell'etichetta POS01.*

…………………………

POS01: 
POS G6A, 10, 32, 15, 120, 78, 93

POS02: 
POS G6A, 67, 47, 32, 153, 23, 33

POS03: 
POS G6A, 34, 37, 122, 162, 84, 28
