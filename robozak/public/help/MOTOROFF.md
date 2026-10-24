**MOTOROFF      모터해제**

Disattiva l'uso dei servomotori.

![](MOTOROFF.files/image001.gif)

 

**Controller della serie MR-C2000)**

 

             -
Sintassi inglese : MOTOROFF *[numero motore]*

             -
Sintassi coreana : 모터해제 *[numero motore]*

 

**Controller della serie MR-C3000)**

 

             -
Sintassi inglese : MOTOROFF *[numero motore] / [gruppo]*

             -
Sintassi coreana : 모터해제 *[numero motore] / [gruppo]*

 

![](MOTOROFF.files/image002.gif)

 

 Il contenuto del comando è identico a quello del comando di impostazione motore (MOTOR). Quando si disattiva l'uso di un motore, la forza (coppia) applicata al motore scompare e i comandi di movimento dei motori usati successivamente non eseguono alcuna azione.

![](MOTOROFF.files/image002.gif)

Il comando MOVE(동작) usato nei controller della serie MR-C3000 aziona il gruppo di motori desiderato tramite la specifica del gruppo. Il comando MOVE24(동작24) è un comando, equivalente a “MOVE G24, ”, che aziona 24 servomotori contemporaneamente; si usa efficacemente per muovere robot composti da più di 16 e fino a 24 servomotori.

 

![](MOTOROFF.files/image003.gif)

 
MOVE24  100, 45, 67, 44, 132, 122, , , , 76, 81, 90
