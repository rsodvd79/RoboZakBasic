**MOVE        Movimento**

Muove più servomotori contemporaneamente.

 

![](MOVE.files/image001.gif)

**Controller della serie MR-C2000)**

 

             -
Sintassi inglese : MOVE *[angolo servomotore0], [angolo servomotore1], ...., [angolo servomotore5]*

             -
Sintassi coreana : 동작 *[angolo servomotore0], [angolo servomotore1], ...., [angolo servomotore5]*

 

**Controller della serie MR-C3000)**

 

             -
Sintassi inglese : MOVE *[gruppo], [angolo servomotore n], ....*

             -
Sintassi coreana : 동작 *[gruppo], [angolo servomotore n], ....*

 

![](MOVE.files/image002.gif)

**Uso del comando MOVE nei controller della serie MR-C2000)**

 

Il comando MOVE muove i servomotori all'angolo desiderato. In questo caso la funzione PWM viene disattivata. ([Attenzione] I comandi MOVE e PWM non possono essere usati insieme nello stesso programma.) In *[angolo servomotore]* si scrive un numero o una costante compresi tra 10 e 190. Se i servomotori usati sono il 1, il 3 e il 4,

 

MOVE  60,    , 100, 120

 

si omette la posizione del servomotore 2, come sopra.
Inoltre, per muovere solo il servomotore 2,

 

MOVE  , 140

 

 omettendo gli angoli del servomotore 1 e dei servomotori da 3 a 6, quei servomotori mantengono l'angolo precedente e si muove solo il servomotore 2, fino a 140 (+40°).

 Poiché il comando MOVE controlla 6 servomotori contemporaneamente, in applicazioni come i robot il controllo del movimento non è semplice. Inoltre, a seconda della struttura di collegamento dei servomotori, il numero che indica l'angolo non fornisce alcuna informazione; collegando il mini controller robot al PC con il cavo di interfaccia e usando in roboBASIC il *“controllo in tempo reale dei servomotori”*, anche un principiante può scrivere facilmente i movimenti del robot con il comando MOVE.

 Se un servomotore è stato modificato per ruotare di 360°, *[angolo servomotore]* indica la velocità di rotazione. Con l'angolo 100 il servomotore si ferma; più il valore è minore di 100, più aumenta la velocità di rotazione a sinistra (rotazione in avanti); più è maggiore di 100, più aumenta la velocità di rotazione a destra (rotazione inversa). Quindi con i servomotori modificati il comando MOVE permette di controllare direzione di rotazione, arresto, velocità di rotazione, ecc.

 Poiché il comando MOVE muove i servomotori, richiede del tempo di movimento. Perciò il mini controller robot, dotato di sistema operativo in tempo reale, esegue il comando successivo contemporaneamente all'avvio del comando MOVE. Per eseguire il comando successivo solo dopo il completamento del movimento dei servomotori, si usa il comando WAIT.

 

![](MOVE.files/image003.gif)

MOVE 100, 50, 140, 120, 80, 40

MOVE 120, , , 160

MOVE , 70, 100

MOVE , , , , , 100

 

**Uso del comando MOVE nei controller della serie MR-C3000)**

 

Nei controller della serie MR-C3000 le porte per i servomotori e quelle per il PWM sono diverse, quindi, a differenza dei controller della serie MR-C2000, possono essere usate contemporaneamente.

 

![](MOVE.files/image003.gif)

Esempio 1)

   
MOVE  G6A, 85, 113, 72, 117, 115, 100

   
MOVE  G6C, 75,    , 96, 123,     , 122

   
MOVE  G8A, 85, 113, 72, 117, 115, 100, 95, 45

 

Esempio 2)

   
MOVE  G24, 85, 113, 72, 117, 115, 100

 

    L'uso di questo comando

 

    MOVE24  85,
113, 72, 117, 115, 100

 

    equivale al comando MOVE24.

![](MOVE.files/image003.gif)

…………………………

MOVEPOS  POS01                        *‘**Esegue la parte del comando POS*

*‘**in corrispondenza dell'etichetta POS01.*

…………………………

POS01: 
POS G6A, 10, 32, 15, 120, 78, 93

POS02: 
POS G6A, 67, 47, 32, 153, 23, 33

POS03: 
POS G6A, 34, 37, 122, 162, 84, 28
