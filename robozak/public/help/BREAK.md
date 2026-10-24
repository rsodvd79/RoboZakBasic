**BREAK Interruzione**

Interrompe temporaneamente l'esecuzione del programma e passa alla modalità di debug. 2000

![](BREAK.files/image001.gif)

             -
Sintassi inglese : BREAK

             -
Sintassi coreana : 중단

 

![](BREAK.files/image002.gif)

Interrompe temporaneamente l'esecuzione del programma, porta roboBASIC in modalità di debug e trasmette al PC, tramite il programma roboBASIC, lo stato della memoria interna del mini controller robot. In questo caso il mini controller robot e il PC devono essere collegati con il cavo di interfaccia; altrimenti il programma resta fermo nello stato di esecuzione interrotta. La modalità di debug è descritta in dettaglio in “Descrizione del programma roboBASIC”.

Il comando “BREAK” funziona solo nei controller della serie MR-C2000 e non funziona nella serie MR-C3000. Tuttavia, anche con la serie MR-C3000 si può usare la modalità di debug per eseguire il programma passo per passo e seguirne l'avanzamento.

 

![](BREAK.files/image003.gif)

...............

BREAK *'Interrompe l'esecuzione del programma.*

   ...............
