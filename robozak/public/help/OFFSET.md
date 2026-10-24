**OFFSET          Offset**

Modifica l'angolo di riferimento (punto zero) del servomotore. 3000

![](OFFSET.files/image001.gif)

**Controller della serie MR-C3000)**

 

- Sintassi inglese : OFFSET *[gruppo], [angolo di zero servomotore n], .....*

- Sintassi coreana : 옵셋 *[gruppo], [angolo di zero servomotore n], .....*

 

![](OFFSET.files/image002.gif)

 In un robot che usa un controller della serie MR-C3000, il valore dell'angolo di movimento varia leggermente a seconda della posizione in cui è montato il servomotore, perciò si usa il comando “ZERO”. Il comando ZERO è memorizzato nella memoria flash del controller e viene letto una sola volta all'accensione, per essere applicato al movimento dei servomotori. Il comando “OFFSET”, a differenza di ZERO, non viene memorizzato in memoria e si usa quando si vuole modificare il punto zero direttamente all'interno del programma.

 Se non si indica l'angolo di zero, viene mantenuto il valore di zero esistente.

 

![모서리가 둥근 직사각형: 명령어 예](OFFSET.files/image003.gif)![](OFFSET.files/image004.gif)

 

OFFSET G6A, 100,
98,  , 102, 100, 106
