**GYROSENSE          Sensibilità giroscopio**

Imposta il sensore giroscopico da applicare a ciascun servomotore collegato. 3000

![](GYROSENSE.files/image001.gif)

             -
Sintassi inglese : GYROSENSE *[specifica gruppo], [sensibilità giroscopio motore n],……*

             -
Sintassi coreana : 자이로감도 *[specifica gruppo], [sensibilità giroscopio motore n],……*

![](GYROSENSE.files/image002.gif)

A un controller serie MR-C3000 si possono collegare in tutto 4 giroscopi; questo comando imposta la sensibilità del sensore giroscopico da applicare a ciascun servomotore collegato al controller.

*[sensibilità giroscopio motore n]* usa un numero o una costante da 0 a 255: 0 non applica affatto il valore in ingresso del sensore giroscopico, e più il valore è alto, più il servomotore reagisce rapidamente al valore in ingresso del sensore giroscopico impostato.

 

![](GYROSENSE.files/image003.gif)

             GYROSENSE
G6A, 100, 100, 255, 255, 50, 50

*‘Imposta la sensibilità del giroscopio a 100 per i servomotori 0 e 1.*

*‘Imposta la sensibilità del giroscopio al massimo (255) per i servomotori 2 e 3.*

*‘Imposta la sensibilità del giroscopio bassa (50) per i servomotori 4 e 5.*
