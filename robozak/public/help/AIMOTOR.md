**AIMOTOR        Impostazione motore AI**

Imposta l'uso dei servomotori AI. 3000

![](AIMOTOR.files/image001.gif)

 

             -
Sintassi inglese : AIMOTOR *SETON/SETOFF/INIT/[numero motore] / [gruppo]*

             -
Sintassi coreana : AI모터설정 *사용설정/사용해제/초기/[numero motore] / [gruppo]*

 

![](AIMOTOR.files/image002.gif)

Il motore AI è un nuovo servomotore dedicato ai robot, sviluppato da Megarobotics. Al suo interno è integrato un chip microcontroller e comunica con il mini controller robot tramite lo standard RS232; permette non solo di controllare l'angolo del motore, ma anche di conoscerne lo stato attuale e di controllare la forza (coppia) del motore tramite i coefficienti di controllo PID (PGAIN, DGAIN). Il mini controller robot può controllare il motore AI con lo stesso concetto di un servomotore.

*I motori AI utilizzabili sono in totale **31**, dalla porta 0 alla porta 30 del mini controller robot*; l'uso si può attivare indicando un motore alla volta con *[numero motore]* oppure tutti insieme con *[gruppo]*. Il modo di usare il comando è lo stesso del comando MOTOR. *[numero motore]* può essere un numero, una costante o una variabile di tipo byte.

A differenza dei normali servomotori, prima di usare un motore AI occorre dichiarare esplicitamente “si usa il motore AI”.

 

- AIMOTOR SETON (AI모터설정 사용설정) : Abilita l'uso del motore AI.

- AIMOTOR SETOFF (AI모터설정 사용해제) : Disattiva l'uso del motore AI.

- AIMOTOR INIT (AI모터설정 초기) : Muove dolcemente il motore AI
nella posizione iniziale.

 

 

 

![](AIMOTOR.files/image003.gif)

             AIMOTOR
INIT                  *‘Inizializza il motore AI*

             AIMOTOR
SETON             *‘Dichiara che in seguito si userà il motore AI*

AIMOTOR
0                  *'Usa il motore AI 0.*

AIMOTOR G6B               *'Usa tutti i motori del gruppo 6B (6-11).*
