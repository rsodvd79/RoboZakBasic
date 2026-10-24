**IF ... THEN ...  만약 ... 이면 ...**

Esegue una valutazione condizionale.

 

![](IF%20TEHN.files/image001.gif)

 

① Nel caso di una condizione singola :

             -
Sintassi inglese : IF *[condizione]* THEN

                                        *[istruzioni da eseguire se la condizione è vera]*

                              
ENDIF

             -
Sintassi coreana : 만약 *[condizione]* 이면

                                        *[istruzioni da eseguire se la condizione è vera]*

                               
만약끝

 

② Nel caso di una condizione composta :

 

             -
Sintassi inglese : IF *[condizione1]* THEN

                                        *[istruzioni da eseguire se la condizione1 è vera]*

                              
ELSEIF *[condizione2]* THEN

                                        *[istruzioni da eseguire se la condizione2 è vera]*

                              
ELSE

                                        *[istruzioni da eseguire se non valgono né la condizione1 né la condizione2]*

                              
ENDIF

             -
Sintassi coreana : 만약 *[condizione1]* 이면

                                        *[istruzioni da eseguire se la condizione1 è vera]*

                               
그렇지않고만약 *[condizione2]* 이면

                                        *[istruzioni da eseguire se la condizione2 è vera]*

                               
아니면

                                        *[istruzioni da eseguire se non valgono né la condizione1 né la condizione2]*

                               
만약끝

 

![](IF%20TEHN.files/image002.gif)

Quando viene eseguita l'istruzione IF ... THEN (만약 ... 이면), si verifica se la condizione dell'IF (만약) iniziale è vera (TRUE). Se è falsa (FALSE), si controlla la condizione di ciascuna istruzione ELSEIF (그렇지않고만약) oppure si esegue l'istruzione ELSE (아니면). Le istruzioni ELSEIF si possono usare in numero a piacere, e in molti casi non vengono usate affatto. In roboBASIC, se la condizione dell'IF è vera, vengono eseguite le istruzioni che seguono il THEN (이면) corrispondente fino all'ELSEIF o all'ELSE successivo; poi l'esecuzione passa al comando successivo all'ENDIF (만약끝), senza ulteriori controlli delle condizioni.

Una delle istruzioni più importanti e indispensabili nei programmi per robot è l'istruzione condizionale (IF..THEN). Per far agire il robot in base a un ingresso esterno, si memorizza il valore dell'ingresso esterno in una variabile, poi si valuta il valore della variabile con l'istruzione condizionale e si programma il robot in modo che si comporti diversamente in base al valore.

 

![](IF%20TEHN.files/image003.gif)

 

① Se la condizione e le istruzioni dell'IF sono semplici, si possono scrivere tutte su una sola riga.

 

IF A > 0 THEN B = 5

IF A < 5 THEN B = 0 ELSE B = 1

 

② La condizione dell'IF può combinare due condizioni contemporaneamente tramite operatori relazionali.

 

IF A > 0 AND A < 5 THEN B = 3

IF A = 7 OR A = 9 THEN B = 1

 

③ Di seguito un esempio di uso più complesso dell'IF.

 

IF A = 1 THEN

   B = 2

   C = 3

ELSEIF A = 3 AND A = 5 THEN

   B = 1

   C = 2

ELSEIF A = 8 THEN

   B = 6

   C = 0

ELSE

   B = 0

   C = 0

ENDIF
