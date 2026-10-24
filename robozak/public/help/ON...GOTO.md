**ON…GOTO      처리…가기**

Esegue un salto condizionato in base al valore di una variabile.

![](ON...GOTO.files/image001.gif)

             -
Sintassi inglese : ON *[variabile]* GOTO *[etichetta], [etichetta], .....*

             -
Sintassi coreana : 처리 *[variabile]* 가기 *[etichetta], [etichetta], .....*

 

![](ON...GOTO.files/image002.gif)

ON...GOTO si usa quando il programma da eseguire cambia in base al valore di *[variabile]*. La ramificazione in base al valore si può realizzare anche con l'istruzione IF, ma ON...GOTO è più semplice e genera un codice oggetto più piccolo rispetto a IF.

 Di seguito è riportato il confronto tra un programma con IF e uno con ON...GOTO che svolgono la stessa funzione.

|  |
| --- |
|  |
|  |  |

 

 Le *[etichetta]* che seguono GOTO corrispondono ai valori di *[variabile]* che parte da 0 e aumenta progressivamente di 1.

*[variabile]* può essere un numero, una costante o una variabile; il numero massimo di *[etichetta]* utilizzabili è 255.
