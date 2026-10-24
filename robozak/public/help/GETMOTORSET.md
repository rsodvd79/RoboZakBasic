**GETMOTORSET 모터입력설정**

Legge il valore corrente dei servomotori e mantiene lo stato attuale. 3000

![](GETMOTORSET.files/image001.gif)

             -
Sintassi inglese : GETMOTORSET *[gruppo], [impostazione lettura motore n],…*

             -
Sintassi coreana : 모터입력설정 *[gruppo], [impostazione lettura motore n],…*

 

![](GETMOTORSET.files/image002.gif)

 Quando al controller della serie MR-C3000 si collegano i servomotori della serie HSR-, dedicati ai robot digitali, è possibile leggere il valore corrente dei servomotori tramite la comunicazione con essi. Alla prima accensione del robot, a causa dell'inizializzazione del controller tutti i servomotori si portano alla posizione iniziale 100 e il movimento del robot risulta brusco; per evitarlo, se si legge il valore corrente dei motori e lo si imposta come valore iniziale dei servomotori, il robot mantiene la propria posa e solo dopo inizia il movimento tramite il comando MOVE (동작), mostrando così un movimento iniziale naturale.

**Nota) L'HSR-5995TG non può usare la funzione di feedback.**

 In *[impostazione lettura motore n]* si usa “0” oppure “1”: con “1” viene letto il valore corrente del servomotore indicato e viene mantenuto lo stato attuale del robot; con “0” il servomotore si porta alla posizione 100, il valore iniziale del controller.

 

![](GETMOTORSET.files/image003.gif)

GETMOTORSET
G8A, 1, 1, 1, 1, 0, 0, 0, 0

*‘I servomotori 0, 1, 2 e 3 mantengono il valore corrente del servomotore all'accensione.*

*‘I servomotori 4, 5, 6 e 7 si portano tutti alla posizione iniziale 100 all'accensione.*
