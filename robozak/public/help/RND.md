**RND     Numero casuale**

Genera un numero casuale.

![](RND.files/image001.gif)

             -
Sintassi inglese : RND

             -
Sintassi coreana : 난수

 

![](RND.files/image002.gif)

Per programmare in modo efficace (casuale) un robot che usa il mini controller robot, è supportato il comando RND; il numero casuale generato è diverso a ogni esecuzione.

 Il numero casuale è un valore arbitrario compreso tra 0 e 255.

 

![](RND.files/image003.gif)

 

DIM A AS BYTE

 

A = RND

BYTEOUT 0, A                 *‘Sulla porta byte 0 viene emesso un valore casuale.*
