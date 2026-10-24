**PTP           Punto a punto**

Attiva/disattiva (ON/OFF) la funzione di controllo del movimento simultaneo dei servomotori.

![](PTP.files/image001.gif)

**Controller serie MR-C2000)**

             -
Sintassi inglese : PTP *[valore di impostazione]*

             -
Sintassi coreana : 점대점 *[valore di impostazione]*

 

**Controller serie MR-C3000)**

             -
Sintassi inglese : PTP *[SETON/SETOFF/ALLON/ALLOFF]*

             -
Sintassi coreana : 점대점 *[사용설정/사용해제/모두설정/모두해제]*

 

![](PTP.files/image002.gif)

Quando si muovono più servomotori con angoli di movimento diversi a velocità costante, i motori terminano il movimento in tempi diversi. Nei mini robot in cui più motori si muovono in modo combinato, come un robot braccio (Arm Robot), questo movimento risulta molto innaturale. Nella robotica (Robot Engineering) esiste la teoria del punto a punto (Point to Point), che calcola l'istante finale dei motori e li fa terminare contemporaneamente, rendendo il movimento naturale; il controller del mini robot integra questa tecnica di controllo del movimento. Il comando PTP (punto a punto) abilita questa tecnica.

 

 

**Controllo PTP (punto a punto) nei controller serie MR-C2000)**

 

 Per *[valore di impostazione]* si usa il numero o la costante 0 (disattivato) oppure 1 (attivato). Usando due servomotori come motore 1 e motore 2, vediamo il seguente esempio di programma.

 

- Caso di movimento normale, innaturale

 

PTP 0

MOVE 100, 100

MOVE 110, 120

 

Spiegazione) Il motore 1 si muove di 10 gradi e il motore 2 di 20 gradi, ma poiché si muovono alla stessa velocità, i motori 1 e 2 si muovono insieme per 10 gradi e il motore 2 percorre da solo i restanti 10 gradi.

 

- Caso di movimento naturale con PTP

 

PTP 1

MOVE 100, 100

MOVE 110, 120

 

Spiegazione) Il motore 1 si muove di 10 gradi e il motore 2 di 20 gradi, ma il motore 1 si muove a metà della velocità del motore 2; i motori 1 e 2 si muovono quindi contemporaneamente e si fermano contemporaneamente.

 

 La traiettoria del movimento dei motori con e senza il comando PTP (punto a punto) è la seguente.

![](PTP.files/image003.jpg)

 

Cioè, nel movimento normale il motore 1 e il motore 2 si muovono insieme fino all'angolo 110, ma dopo 110 si muove solo il motore 2 fino all'angolo 120, e il movimento risulta innaturale.

 Nel movimento con PTP, invece, si esegue il calcolo PTP sugli angoli da percorrere (10 e 20) per i motori 1 e 2, e in base al valore calcolato i motori terminano contemporaneamente, con un movimento naturale.

 

**Controllo PTP (punto a punto) nei controller serie MR-C3000)**

 

 Poiché l'MR-C3000 utilizza molti motori, per i comandi MOVE usati per gruppi si imposta se la funzione PTP debba operare solo sul gruppo specificato oppure su tutti i motori.

 

- PTP SETON (점대점 사용설정) : imposta la funzione PTP per gruppo.

- PTP SETOFF (점대점 사용해제) : disattiva la funzione PTP impostata per gruppo.

- PTP ALLON (점대점 모두설정) : attiva la funzione PTP per tutti i servomotori.

- PTP ALLOFF (점대점 모두해제) : disattiva la funzione PTP per tutti i servomotori.

 

Quando si usa la funzione PTP nei controller serie MR-C3000, usare il *comando "WAIT"* al punto di completamento di ogni movimento di gruppo permette di garantire il completamento del movimento.
