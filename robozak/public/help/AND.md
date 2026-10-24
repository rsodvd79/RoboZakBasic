**Sintassi generale**

roboBASIC si basa sulla sintassi del BASIC generico, quindi in gran parte è uguale o simile al linguaggio BASIC. Qui viene illustrata la sintassi generale di roboBASIC.

 

**Insieme dei caratteri**

 L'insieme dei caratteri di roboBASIC è composto da Hangul (compresi tutti i caratteri coreani/cinesi supportati da Microsoft Windows), lettere inglesi (A-Z, a-z), cifre (0-9) e alcuni caratteri speciali.  Inoltre, i caratteri indicati di seguito hanno un significato particolare in roboBASIC.

 

 

|  |  |
| --- | --- |
| **Carattere** | **Significato** |
| + | Simbolo di addizione |
| - | Simbolo di sottrazione |
| \* | Simbolo di moltiplicazione |
| / | Simbolo di divisione |
| % | Simbolo di resto |
| . | Simbolo di specifica del bit |
| & | Simbolo di base numerica |
| ?? | Simbolo di commento |
| ?? | Simbolo di stringa |
| : | Simbolo di etichetta |
| = | Uguale (operatore relazionale) o simbolo di assegnazione |
| < | Disuguaglianza (minore di) |
| > | Disuguaglianza (maggiore di) |
| << | Simbolo di shift a sinistra dei bit |
| >> | Simbolo di shift a destra dei bit |

**Espressioni e operatori**

 Un'espressione può essere composta dal valore ottenuto combinando costanti, variabili, numeri e così via tramite operatori. Gli operatori eseguono operazioni aritmetiche o logiche sui valori dati. In roboBASIC gli operatori si classificano come segue.

 

|  |  |
| --- | --- |
| **Categoria** | **Funzione** |
| Operatori aritmetici | Eseguono calcoli aritmetici. |
| Operatori relazionali | Confrontano valori numerici. |
| Operatori logici | Confrontano condizioni composte o eseguono manipolazioni di bit. |
| Operatori sui bit | Manipolano i bit o eseguono operazioni sui bit. |

 

**Operatori aritmetici**

 Gli operatori aritmetici sono i simboli usati per eseguire calcoli aritmetici; sono utilizzabili i simboli di operazione aritmetica dei linguaggi BASIC generici, come addizione (+), sottrazione (-), moltiplicazione (\*), divisione (/) e resto (% oppure MOD). In roboBASIC, per le operazioni aritmetiche, ci sono due differenze da tenere presenti rispetto al BASIC generico.

 

Primo, ***gli operatori non hanno precedenza.***

 Non è possibile usare le parentesi ( ) per stabilire a piacere l'ordine di calcolo; indipendentemente dalla priorità degli operatori, il calcolo viene eseguito secondo l'ordine in cui gli operatori compaiono. Ad esempio, A + B \* C nel BASIC generico moltiplica prima B e C e poi somma A al risultato, mentre in roboBASIC somma prima A e B e poi moltiplica il risultato per C.

 

Es.) Con A = 1, B = 2, C = 3.

 

BASIC generico)

                           A
+ B \* C =  1 + 2 \* 3 = 1 + 6 = 7

roboBASIC)

                           A
+ B \* C =  1 + 2 \* 3 = 3 \* 3 = 9

 

Secondo, ***usando calcoli aritmetici troppo complessi possono verificarsi errori imprevisti.***

 

 In questo caso basta suddividere il calcolo in 2-3 passaggi.

 

Esempio di calcolo aritmetico)

 

  D = A \* B + C                            (uso corretto)

  F = A \* B / C \* D + E   (evitare calcoli troppo complessi.)

 

Terzo, ***poiché roboBASIC supporta solo i tipi byte e intero, nelle divisioni la parte decimale del risultato viene ignorata.***

 

 Il resto si calcola con l'operatore di resto % oppure MOD; l'operatore di resto fornisce come risultato il resto e non il quoziente.

 

**Operatori relazionali**

 Gli operatori relazionali si usano per confrontare due valori. Il risultato del confronto è "vero" (TRUE) o "falso" (FALSE), e viene usato nell'istruzione IF (se) per controllare il flusso del programma.

 

|  |  |  |
| --- | --- | --- |
| **Operatore** | **Relazione** | **Espressione** |
| = | Uguale. | X = Y |
| <> | Diverso. | X <> Y |
| < | Minore di. | X < Y |
| > | Maggiore di. | X > Y |
| <= | Minore o uguale a. | X <= Y |
| >= | Maggiore o uguale a. | X >= Y |

 

 Se in una stessa espressione sono combinati operatori aritmetici e operatori logici, le operazioni aritmetiche vengono eseguite per prime.

 

**Operatori logici**

 Gli operatori logici si usano per confrontare condizioni composte e restituiscono come risultato "vero" o "falso". Questo risultato può essere usato nell'istruzione IF (se) per controllare il flusso del programma.

|  |  |
| --- | --- |
| **Operatore** | **Significato** |
| AND | Prodotto logico |
| OR | Somma logica |
| XOR | Somma logica esclusiva |

 

 Ogni operatore produce i risultati mostrati sotto. Nella tabella "T" indica vero e "F" indica falso.

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| **Valori di X, Y** | | **Valore restituito dall'operatore logico (risultato)** | | |
| **X** | **Y** | **X AND Y** | **X OR Y** | **X XOR Y** |
| T | T | T | T | F |
| T | F | F | T | T |
| F | T | F | T | T |
| F | F | F | F | F |

 

**Operatori sui bit**

 Gli operatori sui bit eseguono operazioni su ciascun bit delle variabili, utilizzate dal mini controller robot per gestire comodamente i bit attraverso le porte di ingresso/uscita. Le operazioni su tutti i bit comprendono la somma di bit (OR), il prodotto di bit (AND) e la somma di bit esclusiva (XOR); in roboBASIC sono inoltre presenti i simboli di operazione per spostare (shift) tutti i bit a sinistra (<<) o a destra (>>) e l'operatore "." per specificare un bit particolare di una variabile.

 Se il valore di A è 33 (in binario 00100001) e il valore di B è 15 (in binario 00001111), gli operatori sui bit producono i risultati della tabella seguente.

|  |  |
| --- | --- |
| **Operatore** | **Risultato** |
| A AND B | 1 (00000001) |
| A OR B | 47 (00101111) |
| A XOR B | 46 (00101110) |
| A << 1 | 66 (01000010) |
| A >> 1 | 16 (00010000) |
| A.0 | 1 (bit 0 di A) |

 Quando nella stessa istruzione sono usati più operatori, le operazioni vengono eseguite nel seguente ordine:

 

① Operatori aritmetici / operatori sui bit

② Operatori relazionali

③ Operatori logici

 

**Numeri, variabili/costanti e altre spiegazioni sulla sintassi**

 Poiché roboBASIC è un linguaggio BASIC creato per il controllo dell'hardware, non supporta le variabili o costanti relative alle stringhe né gli array di variabili usati nel BASIC generico.

 

**Tipi numerici**

 I tipi numerici sono byte (BYTE) e intero (INTEGER); l'intervallo di valori per ciascun tipo è riportato nella tabella.

|  |  |  |
| --- | --- | --- |
| **Tipo numerico** | **Dimensione** | **Intervallo** |
| Byte (BYTE) | 1 byte (8 bit) | 0-255 |
| Intero (INTEGER) | 2 byte (16 bit) | 0-65535 |

 

 ***roboBASIC non supporta i numeri negativi.*** Perciò, anteponendo a un numero il segno + o -, si eseguono operazioni errate o si genera un errore. Inoltre, se durante un calcolo una variabile dichiarata esce dall'intervallo del suo tipo, non viene generato alcun errore ma il risultato non è corretto: occorre quindi dichiarare sempre il tipo numerico con un intervallo adeguato.

 

**Basi numeriche**

 

 Poiché roboBASIC è un linguaggio per il controllo dell'hardware, oltre alla consueta notazione decimale, nella scrittura dei programmi può essere comodo esprimere i numeri in altre basi, come l'esadecimale. roboBASIC permette varie notazioni numeriche: binaria (Bin), ottale (Oct), decimale (Dec) ed esadecimale (Hex). Per usare una base specifica si dichiara la base con il simbolo "&"; per i numeri decimali non serve alcuna dichiarazione.

 Il numero decimale 61 si esprime nelle varie basi come nella tabella seguente; se si esce dall'intervallo di cifre utilizzabili nella base, si verifica un errore, quindi occorre usare sempre cifre valide.

 

|  |  |  |  |
| --- | --- | --- | --- |
| **Base** | **Notazione** | **Cifre utilizzabili** | **Esempio** |
| Binaria | &B | 0, 1 | &B111101 |
| Ottale | &O | 0, ... , 7 | &O75 |
| Decimale | nessuna | 0, ... , 9 | 61 |
| Esadecimale | &H | 0, ... , 9, A, ... F | &H3D |

 

**Costanti e variabili**

 Una costante è un valore che non cambia durante l'esecuzione del programma. roboBASIC permette di definire costanti numeriche di tipo byte e intero. Il tipo della costante è determinato automaticamente dall'intervallo del numero usato nella definizione, e una costante già definita non può essere ridefinita. Poiché la costante definita viene sostituita direttamente con il suo valore nei punti in cui è usata, non influisce affatto sulla dimensione del programma; definire come costanti i numeri usati di frequente rende più comoda la modifica del programma.

 Un esempio di definizione di costanti usate principalmente per il controllo dell'hardware del mini controller robot è il seguente.

 

CONST OFF = 0

CONST 모터\_1 = 3

CONST 모터\_1속도 =
200

 

 Una variabile è il nome di un'area di memoria per i dati usati dal programma. Il numero di variabili utilizzabili nel mini controller robot è limitato, quindi, in base allo scopo, occorre dichiarare le variabili con il tipo che ne minimizza la dimensione.

 Un esempio di dichiarazione di variabili usate di frequente nel mini controller robot è il seguente.

 

DIM 모터\_1\_지연 AS
INTEGER

DIM 센서\_왼쪽 AS
BYTE

 

 Nel dichiarare variabili e costanti occorre rispettare le seguenti regole.

 

 Primo, nei nomi di variabili/costanti si possono usare caratteri Hangul (hanja), lettere inglesi, cifre e "\_"; il primo carattere deve essere obbligatoriamente una lettera Hangul o inglese.

 

 Secondo, la lunghezza del nome di variabile/costante non può superare i 64 caratteri. (Hangul e lettere inglesi contano indifferentemente come 1 carattere ciascuno.)

 

 Terzo, non si può dichiarare più volte lo stesso nome di variabile/costante; poiché maiuscole e minuscole non sono distinte, occorre fare attenzione.

 

 Quarto, nel dichiarare una costante occorre specificare un valore non superiore a 65535, il limite del tipo intero; se si supera tale valore si verifica un errore.

 

**Specifica del bit**

 roboBASIC permette di manipolare le variabili bit per bit. Per operare sui singoli bit si usa l'operatore di specifica del bit ("."); per le variabili di tipo byte si possono specificare i bit da 0 a 7, per le variabili di tipo intero i bit da 0 a 15. Nella specifica del bit si deve usare sempre un numero o una costante. Un esempio di specifica del bit è il seguente.

 

   DIM A AS INTEGER

   CONST BIT\_2 = 2

 

   A.1 = 1

   A.BIT\_2 = 0

   A.3 = IN(1)     *‘Legge il valore della porta 1 e lo mette nel bit 3 della variabile intera A*

   OUT 2,
A.1                *‘Emette sulla porta 2 il valore del bit 1 della variabile A*

 

**Commenti**

 Per scrivere e gestire il programma in modo efficace, è necessario inserire qua e là spiegazioni sul programma. Per i commenti si usa all'inizio ***il simbolo di commento (‘) oppure il comando REMARK (설명)***, e non hanno alcun effetto sull'esecuzione del programma.

 

**Istruzione di assegnazione (=)**

 L'istruzione di assegnazione si usa per assegnare un valore a una variabile e usa semplicemente il simbolo "=" della matematica. A sinistra del simbolo di assegnazione (=) c'è sempre una variabile; a destra possono esserci variabili, stringhe, espressioni di calcolo o funzioni.

 

 Di seguito alcuni esempi di istruzioni di assegnazione.

 

A =
B                          *‘Assegnazione tra variabili*

A.1 =
1                        *‘Assegnazione con specifica del bit*

A = AD (0)   
                  *‘Assegnazione di una funzione*

A = 3 \* 2 -
1                    *‘Assegnazione di un'espressione numerica*

A = C + B -
A                  *‘Assegnazione di un'espressione con variabili*

A =
"1"                         *‘Assegnazione del codice ASCII di un carattere*

 

 

**Etichette di riga**

 Le etichette di riga (Label) si usano per indicare una posizione nel programma e possono essere lettere o numeri. Per assegnare i nomi alle etichette valgono alcune regole.

 

 Primo, il nome di un'etichetta letterale non può superare i 64 caratteri e il primo carattere deve essere obbligatoriamente Hangul/inglese.

 

 Secondo, dopo il nome di un'etichetta letterale deve essere obbligatoriamente apposto il simbolo di etichetta (:).

 

 Terzo, il nome di un'etichetta numerica può essere solo un numero compreso tra 0 e 65535 e non si appone il simbolo di etichetta.

 

 Quarto, non si può dichiarare più volte lo stesso nome di etichetta; poiché maiuscole e minuscole non sono distinte, occorre fare attenzione.

 

 Le etichette si usano principalmente nei comandi di controllo del flusso del programma, come GOTO (가기) o GOSUB (호출). Un esempio di uso delle etichette è il seguente.

 

DIM A AS INTEGER

 

**START:**

          A
= IN(0)

          IF
A = 0 THEN

GOTO **START**

             ELSE

                  
GOSUB 10

          END

          GOTO
**START**

 

**10**OUT 1, 0

DELAY 100

OUT 1, 1

          RETURN
