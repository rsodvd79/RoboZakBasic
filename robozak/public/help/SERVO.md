**SERVO       Servo**

Muove un servomotore.

![](SERVO.files/image001.gif)

             -
Sintassi inglese : SERVO *[numero servomotore],[angolo servomotore]*

             -
Sintassi coreana : 서보 *[numero servomotore],[angolo servomotore]*

 

![](SERVO.files/image002.gif)

Muove il servomotore all'angolo desiderato. Nei controller della serie MR-C2000 la funzione PWM viene disattivata. *[numero servomotore]* è il numero della porta a cui è collegato il servomotore; per *[angolo servomotore]* si possono usare un numero compreso tra 10 e 190, una costante o una variabile di tipo byte.

 

![](SERVO.files/image003.gif)

 

Esempio 1)

             SERVO
1,130                                 *'
Muove il servomotore 1 alla posizione 130 (angolo +30 gradi)*

Esempio 2)

             DIM
I AS BYTE

 

             FOR 
I = 10 TO 190

                           SERVO
4, I

                           DELAY
100

             NEXT
I
