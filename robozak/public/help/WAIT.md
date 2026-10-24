**WAIT    Attendi**

Attende il termine dell'esecuzione del comando.

![](WAIT.files/image001.gif)

             -
Sintassi inglese : WAIT

             -
Sintassi coreana : 대기

 

![](WAIT.files/image002.gif)

Il sistema operativo di controllo integrato nel mini controller robot è un software moderno basato sul concetto di elaborazione in tempo reale (Real-Time) tramite timer. Perciò, quando si esegue un comando che richiede un certo tempo, come il movimento di un servomotore, il programma non si ferma fino al completamento del comando, ma esegue il comando successivo subito dopo l'avvio del precedente. Se invece serve che il comando successivo venga eseguito solo dopo il completo termine del precedente, si usa il comando WAIT.

 Il comando “MOVE” usato per i movimenti del robot aziona più servomotori contemporaneamente, quindi WAIT viene usato molto spesso per attendere che tutti i servomotori abbiano terminato il movimento.

 

![](WAIT.files/image003.gif)

I seguenti esempi sono: un programma che muove 6 servomotori agli angoli impostati e, dopo che tutti si sono fermati, attiva le uscite 7 e 8 (esempio 1); e un programma che attiva l'uscita 7 durante il movimento dei servomotori e l'uscita 8 dopo il loro arresto (esempio 2).

 

Esempio 1)

 

MOVE 120, 100, 140, 90, 70, 150

WAIT

OUT 7, 1

OUT 8, 1

 

Esempio 2)

 

MOVE 120, 100, 140, 90, 70, 150

OUT 7, 1

WAIT

OUT 8, 1
