**INIT           초기**

Imposta la posa iniziale di movimento del robot. 3000

![](INIT.files/image001.gif)

             -
Sintassi inglese : INIT *[gruppo], [angolo motore n],……*

             -
Sintassi coreana : 초기 *[gruppo], [angolo motore n],……*

 

![](INIT.files/image002.gif)

Quando si monta il controller della serie MR-C3000 su un robot composto da servomotori, alla prima accensione tutti i servomotori vengono inizializzati alla posizione 100 e solo dopo inizia il movimento del robot tramite il comando MOVE (동작). Poiché questo movimento iniziale provoca movimenti bruschi dei motori non appena si dà alimentazione, c'è spesso il rischio di danneggiare il robot. Per evitarlo, specificando la posa iniziale con il comando INIT (초기), all'accensione i motori si portano nelle posizioni indicate da INIT anziché nella posizione iniziale 100, evitando movimenti bruschi del robot.

*Il comando INIT (초기) svolge una funzione simile al comando GETMOTORSET (모터입력설정); in generale, per i servomotori analogici (serie HS- di HITEC) si usa il comando INIT, mentre per i servomotori digitali dedicati ai robot (serie HSR- di HITEC) si usa il comando GETMOTORSET.*

 

![](INIT.files/image003.gif)

  INIT
G8A, 100, 45, 67, 44, 132, 122, 76, 81
