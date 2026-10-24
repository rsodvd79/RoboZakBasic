**‘$LIMIT      ‘$제한**

Limita l'intervallo di movimento dei servomotori usati nel programma in fase di scrittura. 3000

![](LIMIT.files/image001.gif)

             -
Sintassi inglese : '$LIMIT *[numero motore], [valore minimo],
[valore massimo]*

             -
Sintassi coreana : '$제한 *[numero motore], [valore minimo], [valore massimo]*

 

![](LIMIT.files/image002.gif)

Questo comando LIMIT (제한) ha effetto nella schermata di controllo motori in tempo reale di roboBASIC. Serve cioè a evitare che i motori del robot si danneggino uscendo dall'intervallo limite mentre si controlla in tempo reale un robot composto da servomotori. Non ha quindi alcuna relazione con il download del programma sul controller e la sua esecuzione.

Per *[numero motore]* si usa il numero del servomotore da 0 a 31; per *[valore minimo]* e *[valore massimo]* si usano valori dell'intervallo limite del motore compresi tra 10 e 190. Inizialmente tutti i motori hanno un intervallo limite di 10-190.

 

![](LIMIT.files/image003.gif)

 

'$LIMIT 
0, 50, 100      *'Limita l'intervallo dell'angolo di movimento del servomotore 0 tra 50 e*

                               
*'100.*
