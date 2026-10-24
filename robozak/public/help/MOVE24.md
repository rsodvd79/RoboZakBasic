**MOVE24          Movimento 24**

Muove 24 servomotori. 3000

![](MOVE24.files/image001.gif)

             -
Sintassi inglese : MOVE24 *[angolo motore 0],………,[angolo motore 23]*

             -
Sintassi coreana : 동작24 *[angolo motore 0],………,[angolo motore 23]*

 

![](MOVE24.files/image002.gif)

Il comando MOVE (동작) dei controller serie MR-C3000 muove il gruppo di motori desiderato tramite la specifica del gruppo. Il comando MOVE24 (동작24) muove contemporaneamente 24 servomotori, come "MOVE G24, ", ed è utile per muovere robot composti da un numero di servomotori compreso tra 16 e 24.

 

![](MOVE24.files/image003.gif)

 
MOVE24  100, 45, 67, 44, 132, 122, , , , 76, 81, 90
