**SPEED       속도**

Imposta la velocità di movimento dei servomotori.

![](SPEED.files/image001.gif)

             -
Sintassi inglese : SPEED *[velocità motore]*

             -
Sintassi coreana : 속도 *[velocità motore]*

 

![](SPEED.files/image002.gif)

Il comando SPEED (속도) imposta la velocità di movimento quando i servomotori vengono mossi con il comando MOVE (동작). *Nei controller della serie MR-C2000, per [velocità motore] si possono usare numeri o costanti da 1 a 15; nei controller della serie MR-C3000 è possibile usare anche variabili di tipo byte.* La velocità di movimento predefinita dei motori è 3 e, più il numero è grande, più il movimento è veloce. Se si usa una velocità troppo alta, poiché i motori si muovono rapidamente possono verificarsi incidenti imprevisti. È quindi necessario impostare una velocità di movimento dei motori adeguata.

*Nella serie MR-C3000, dopo aver impostato HIGHSPEED SETON, la velocità diventa triplicata.*

**Ciò rimane valido fino a HIGHSPEED SETOFF.**

![](SPEED.files/image003.gif)

Esempio 1)

   SPEED 7                                  *'Imposta la velocità del motore a 7.*

Esempio 2)

   DIM STEP\_SPEED
AS BYTE    *'Dichiara una variabile chiamata STEP\_SPEED.*

   STEP\_SPEED = 15                   *'Imposta il valore della variabile STEP\_SPEED a 15.*

   SPEED
STEP\_SPEED               *'Imposta il valore di SPEED*

*'con il valore della variabile STEP\_SPEED.*

![](SPEED.files/image002.gif)

Il comando MOVE (동작) usato nei controller della serie MR-C3000 muove il gruppo di motori desiderato tramite la specifica del gruppo. Il comando MOVE24 (동작24) è un comando che muove contemporaneamente 24 servomotori, come in “MOVE G24, ”, ed è utile per muovere robot composti da 16 a 24 servomotori.

 

![](SPEED.files/image003.gif)

 
MOVE24  100, 45, 67, 44, 132, 122, , , , 76, 81, 90
