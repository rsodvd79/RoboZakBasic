**OUT     출력**

Invia un segnale digitale alla porta.

![](OUT.files/image001.gif)

             -
Sintassi inglese : OUT *[numero porta]*, *[valore di uscita]*

             -
Sintassi coreana : 출력 *[numero porta]*, *[valore di uscita]*

 

![](OUT.files/image002.gif)

Invia un segnale tramite una porta del controller miniROBOT. Inviando il valore 0 (LOW) si ottiene un segnale di 0V; inviando il valore 1 (HIGH) si ottiene un segnale di +5V. Come [valore di uscita] si possono usare un numero (0 o 1), una costante, una variabile, ecc.; poiché per la porta di uscita sono validi solo i valori 0 o 1, come [valore di uscita] si può usare anche la specifica di un bit di una variabile.

 

![](OUT.files/image003.gif)

Il seguente esempio è scritto per un circuito di prova dell'ingresso/uscita delle porte, con un pulsante (Push Button) collegato alla porta 0 e un LED collegato alla porta 3.

 

DIM A AS INTEGER

 

A =
0              
                       *'Inizializza la variabile A*

START: A =
IN(0)                                   *'Legge lo stato del pulsante*

                              
                    *'Si può usare anche A.0 = IN(0).*

IF A = 1 THEN                             *'Se il pulsante non è premuto*

   OUT 3,
0                              *'Spegne il LED*

ELSE                
                    *'Altrimenti, se il pulsante è premuto*

   OUT 3,
1                              *'Accende il LED*

ENDIF

GOTO START                            *'Controlla di nuovo il pulsante*
