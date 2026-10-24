**CSON()      커서보임**

**CSOFF()          커서감춤**

Mostra o nasconde il cursore sul modulo LCD.

![](CSON.files/image001.gif)

             -
Sintassi inglese : CSON   /   CSOFF

             -
Sintassi coreana : 커서보임   /   커서감춤

 

![](CSON.files/image002.gif)

 CSON(커서보임) e CSOFF(커서감춤) sono i comandi che mostrano o nascondono il cursore quando si visualizza una stringa sul modulo LCD. Il cursore del modulo LCD indica la posizione in cui verrà visualizzata la stringa successiva; normalmente lo si usa nascosto. Quando il modulo LCD viene inizializzato, il cursore non è visibile. Questi comandi servono anche per attivare/disattivare (ON/OFF) lo scorrimento (scroll) dello schermo.

 

![](CSON.files/image003.gif)

![](CSON.files/image004.jpg)LCDINIT

CSON

PRINT
"CURSOR ON"
