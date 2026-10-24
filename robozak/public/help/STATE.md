**STATE()          Stato di uscita()**

Legge il valore dello stato di uscita corrente di una porta di uscita.

![](STATE.files/image001.gif)

             -
Sintassi inglese : STATE (*[numero porta]*)

             -
Sintassi coreana : 출력상태 (*[numero porta]*)

 

![](STATE.files/image002.gif)

Dopo aver inviato un segnale tramite una porta di uscita, per conoscere lo stato del valore di quella porta non si deve usare di nuovo la funzione di ingresso (IN), ma la funzione STATE (출력상태), che rileva lo stato della porta di uscita.

 

![](STATE.files/image003.gif)

 Esempio di programma che verifica lo stato di uscita della porta 1 del controller del mini robot.

 

DIM A AS BYTE

 

OUT 1, 1

A = STATE(1)*'
A = 1*

OUT 1, 0

A = STATE(1)*'
A = 0*
