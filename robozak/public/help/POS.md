**MOVEPOS        동작위치**

**POS                     모터위치**

Specifica una posa particolare dei motori e la esegue. 3000

![](POS.files/image001.gif)

             -
Sintassi inglese :  MOVEPOS *[etichetta di riga]*

                            
…………

                     
[etichetta di riga]: POS *[gruppo], [angolo motore n],……*

 

             -
Sintassi coreana :  동작위치 *[etichetta di riga]*

                            
…………

                     
[etichetta di riga]: 모터위치 *[gruppo], [angolo motore n],……*

 

![](POS.files/image002.gif)

Nei controller della serie MR-C3000, quando la posa del robot realizzata con MOVE (동작) deve essere richiamata in altre parti del programma, la si definisce con il comando POS (모터위치) e una *[etichetta di riga]* e la si chiama dove necessario con il comando MOVEPOS (동작위치). I comandi MOVEPOS e POS permettono di scrivere in modo conciso i programmi roboBASIC che ripetono spesso le stesse pose e ne facilitano la modifica.

 

![](POS.files/image003.gif)

…………………………

MOVEPOS  POS01                        *‘Esegue la parte del comando POS*

*‘in corrispondenza dell'etichetta POS01.*

…………………………

POS01: 
POS G6A, 10, 32, 15, 120, 78, 93

POS02: 
POS G6A, 67, 47, 32, 153, 23, 33

POS03: 
POS G6A, 34, 37, 122, 162, 84, 28
