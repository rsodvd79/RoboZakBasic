**FOR ... NEXT   반복 ... 다음**

Esegue un'iterazione per un numero di volte stabilito.

![](FOR%20NEXT%20.files/image001.gif)

 

             -
Sintassi inglese : FOR *[variabile di ciclo]* = *[inizio]* TO *[fine]*

                                        *[istruzioni da ripetere]*

                               
NEXT [variabile di ciclo]

             -
Sintassi coreana : 반복 *[variabile di ciclo]* = *[inizio]* 부터 *[fine]*

                                        *[istruzioni da ripetere]*

                               
다음 *[variabile di ciclo]*

 

![](FOR%20NEXT%20.files/image002.gif)

 

[variabile di ciclo] è la variabile che conta il numero di ripetizioni, [inizio] è il valore iniziale della variabile di ciclo e [fine] è il suo valore finale.
[inizio] e [fine] possono essere numeri, costanti, variabili, ecc.

 In roboBASIC, [fine] della [variabile di ciclo] deve essere maggiore di [inizio], e la [variabile di ciclo] viene sempre incrementata di 1. Quando si usa l'istruzione FOR...NEXT in roboBASIC ci sono alcune regole da rispettare.

 

① All'interno di un'istruzione FOR...NEXT si può usare un'altra istruzione FOR...NEXT.

 

FOR I = 1 TO 10

   FOR J = 1 TO 5

      ..........

   NEXT J

NEXT I

 

② Quando si usano più istruzioni FOR...NEXT, non si deve invertire l'ordine delle NEXT [variabile di ciclo].

 

FOR I = 1 TO 10

   FOR J = 1 TO 5

      ..........

   NEXT I

NEXT J

 

 Nell'esempio precedente l'ordine delle variabili di ciclo di NEXT è invertito: la generazione del codice oggetto non dà errori, ma quando il mini robot controller interpreta ed esegue il programma possono verificarsi risultati errati.

 

③ Dall'interno di un'istruzione FOR…NEXT si può uscire verso l'esterno, ma dall'esterno non si può entrare all'interno di un'istruzione FOR…NEXT.

![](FOR%20NEXT%20.files/image003.jpg)

 

 

④ Il valore delle variabili usate in [variabile di ciclo], [inizio] e [fine] non deve essere modificato arbitrariamente dalle istruzioni all'interno del FOR…NEXT.

 

![](FOR%20NEXT%20.files/image004.gif)

 

 Esempio di programma che, con un LED collegato alla porta 0 del controller, lo fa lampeggiare 5 volte.

 

             DIM
A AS BYTE                *‘Dichiara la variabile usata nel ciclo*

 

             FOR
A = 1 TO 5                *‘Le ripetizioni sono 5, da 1 a 5*

                           OUT
0, 0             *‘Accende (ON) il LED della porta 0*

                           DELAY
100          *‘Ritardo per un certo tempo*

                           OUT
0, 1             *‘Spegne (OFF) il LED della porta 0*

                           DELAY
100          *‘Ritardo per un certo tempo*

             NEXT
A
