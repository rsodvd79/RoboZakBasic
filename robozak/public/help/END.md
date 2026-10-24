**END     Fine**

Termina l'esecuzione del programma.

![](END.files/image001.gif)

 

             -
Sintassi inglese : END

             -
Sintassi coreana : 끝

 

![](END.files/image002.gif)

 

Il mini controller robot avvia subito l'esecuzione del programma salvato nella EEPROM 2 secondi dopo l'alimentazione. Un normale programma per controller è strutturato in modo da essere eseguito indefinitamente finché è alimentato. Perciò, nella maggior parte dei casi il programma è composto da azioni condizionate dagli ingressi esterni; tuttavia, quando si fa programmazione strutturata, ad esempio scrivendo subroutine a metà programma o dopo le istruzioni di esecuzione, si usa il comando END per indicare la fine dell'esecuzione del programma.

 

![](END.files/image003.gif)

 

① Termina l'esecuzione del programma durante il suo svolgimento.

 

DIM A AS BYTE

 

START: A =
IN(0)

IF A = 1 THEN END

...........

GOTO START

 

② Si possono scrivere subroutine strutturate.

 

DIM A AS BYTE

 

A = BYTEIN(0)

IF A = 1 THEN

   GOSUB L1

ELSEIF A = 3 THEN

   GOSUB L2

ELSEIF A = 4 THEN

   GOSUB L3

ELSE

   GOSUB L4

ENDIF

...........

END

 

L1:
..............

RETURN

 

L2:
.............

RETURN

 

L3:
............

RETURN

 

L4: ...........

RETURN
