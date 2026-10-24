**ACCEL       가속도**

Imposta l'accelerazione del movimento dei servomotori. 2000

![](ACCEL.files/image001.gif)

             -
Sintassi inglese : ACCEL *[accelerazione motore]*

             -
Sintassi coreana : 가속도 *[accelerazione motore]*

 

![](ACCEL.files/image002.gif)

|  |
| --- |
|  |
|  |  |

  
Il comando ACCEL imposta la variazione di velocità, cioè l'accelerazione, dall'inizio del movimento fino al raggiungimento di una velocità costante quando si muove un motore con il comando MOVE.

 

 

 *[accelerazione motore]* è un numero o una costante da 0 a 15. L'accelerazione predefinita del movimento del motore è 3: più il numero è grande, più l'accelerazione è elevata e più il movimento è brusco.

Quando si avvia per la prima volta il movimento dei motori, la posizione dei motori fermi non è nota, quindi usando subito il comando MOVE i motori si muoveranno bruscamente. In tal caso, impostando accelerazione e velocità basse, il primo comando MOVE produce un movimento dolce: è quindi efficace usare i comandi ACCEL e SPEED.

 *Nei controller della serie MR-C3000 il comando ACCEL non viene utilizzato.*

 

![](ACCEL.files/image004.gif)

ACCEL 7    *'Imposta a 7 l'accelerazione del movimento dei servomotori.*
