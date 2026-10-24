**GYROSET        자이로설정**

Imposta il sensore giroscopico da applicare a ciascun servomotore collegato. 3000

![](GYROSET.files/image001.gif)

             -
Sintassi inglese : GYROSET *[gruppo], [giroscopio motore n],……*

             -
Sintassi coreana : 자이로설정 *[gruppo], [giroscopio motore n],……*

 

![](GYROSET.files/image002.gif)

Al controller della serie MR-C3000 si possono collegare in tutto 4 giroscopi; si imposta la porta del sensore giroscopico da applicare a ciascun servomotore collegato al controller.

*[giroscopio motore n]* è il numero di porta (da 1 a 4) del sensore giroscopico da applicare al servomotore corrispondente. Se si imposta 0, l'ingresso del sensore giroscopico non viene utilizzato.

 

![](GYROSET.files/image003.gif)

             GYROSET
G6B, 1, 1, 2, 2, 0, 0

*‘Il servomotore 6 riceve ed elabora il valore del sensore giroscopico 1.*

*‘Il servomotore 7 riceve ed elabora il valore del sensore giroscopico 1.*

*‘Il servomotore 8 riceve ed elabora il valore del sensore giroscopico 2.*

*‘Il servomotore 9 riceve ed elabora il valore del sensore giroscopico 2.*

*‘I servomotori 10 e 11 non usano il sensore giroscopico.*
