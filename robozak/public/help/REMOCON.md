**REMOCON()          리모콘()**

Legge il valore del telecomando a infrarossi dalla porta di conversione AD 7. 3000

![](REMOCON.files/image001.gif)

             -
Sintassi inglese : REMOCON (*[tipo di telecomando]*)

             -
Sintassi coreana : 리모콘 (*[tipo di telecomando]*)

 

![](REMOCON.files/image002.gif)

Legge il valore del tasto premuto sul telecomando a infrarossi tramite la porta di conversione AD 7 dei controller della serie MR-C3000. *[tipo di telecomando]* si usa in base ai tipi di telecomando definiti a partire dal numero 0; i tipi di telecomando utilizzabili possono aumentare a seconda della versione dei controller della serie MR-C3000. Per i dettagli, si vedano gli esempi con il telecomando a infrarossi più avanti.

 

|  |
| --- |
|  |
|  |  |

  
Telecomando tipo 0 (modello : MR-IR2)

 

![](REMOCON.files/image004.gif)

DIM a AS BYTE              *'Variabile che riceve il valore ricevuto*

MAIN:                                          *'Etichetta MAIN per leggere continuamente il valore del telecomando*

a = REMOCON(0)                         *'Mette nella variabile a il valore ricevuto dal telecomando.*

ON a GOTO
MAIN,KEY1,KEY2,KEY3,KEY4 *'Se non c'è alcun valore ricevuto, va a MAIN*

GOTO MAIN                                *'Salta all'etichetta MAIN.*

END

KEY1:                                          *'Gestione se il valore ricevuto è 1*

          
...................

GOTO MAIN                               *'Torna allo stato di ricezione*

KEY2:                                          *'Gestione se il valore ricevuto è 2*

          
...................

GOTO MAIN                                *'Torna allo stato di ricezione*

KEY3:                                          *'Gestione se il valore ricevuto è 3*

          
...................

GOTO MAIN                                *'Torna allo stato di ricezione*

KEY4:                                          *'Gestione se il valore ricevuto è 4*

          
...................

GOTO MAIN                                *'Torna allo stato di ricezione*

 

  

 

![모서리가 둥근 직사각형: 적외선 리모콘 1 번](REMOCON.files/image005.gif)![](REMOCON.files/image006.gif)

Il nuovo telecomando a infrarossi tipo 1, a differenza del telecomando precedente che poteva controllare un solo robot, permette di impostare fino a 4 diversi valori di ID dei robot controllabili, rendendo possibili competizioni, giochi, ecc. con più robot. È particolarmente efficace per controllare il robot bipede “Robonova (ROBONOVA-I)”.

Ha l'aspetto mostrato nella figura seguente, e i valori di codice per ciascun pulsante sono i seguenti.

 

![](REMOCON.files/image007.gif)

Aspetto del telecomando MR-IR3

 

 

 

Tabella dei codici dei tasti del telecomando MR-IR3

 

|  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- |
| **Numero tasto** | **Funzione** | **Valore del tasto in base all'ID** | | | |
| **ID-1** | **ID-2** | **ID-3** | **ID-4** |
| K01 | P1 | 25 | 57 | 89 | 121 |
| K02 | P2 | 19 | 51 | 83 | 115 |
| K03 | POWER | 16 | 48 | 80 | 112 |
| K04 | 1 | 1 | 33 | 65 | 97 |
| K05 | 2 | 2 | 34 | 66 | 98 |
| K06 | 3 | 3 | 35 | 67 | 99 |
| K07 | 4 | 4 | 36 | 68 | 100 |
| K08 | 5 | 5 | 37 | 69 | 101 |
| K09 | 6 | 6 | 38 | 70 | 102 |
| K10 | 7 | 7 | 39 | 71 | 103 |
| K11 | 8 | 8 | 40 | 72 | 104 |
| K12 | 9 | 9 | 41 | 73 | 105 |
| K13 | \* | 22 | 54 | 86 | 118 |
| K14 | 0 | 10 | 42 | 74 | 106 |
| K15 | # | 24 | 56 | 88 | 120 |
| K16 | ▲ | 11 | 43 | 75 | 107 |
| K17 | ◀ | 14 | 46 | 78 | 110 |
| K18 | ■ | 26 | 58 | 90 | 122 |
| K19 | ▶ | 13 | 45 | 77 | 109 |
| K20 | ▼ | 12 | 44 | 76 | 108 |
| K21 | A | 15 | 47 | 79 | 111 |
| K22 | △ | 21 | 53 | 85 | 117 |
| K23 | B | 20 | 52 | 84 | 116 |
| K24 | ◁ | 28 | 60 | 92 | 124 |
| K25 | □ | 29 | 61 | 93 | 125 |
| K26 | ▷ | 30 | 62 | 94 | 126 |
| K27 | C | 17 | 49 | 81 | 113 |
| K28 | ▽ | 31 | 63 | 95 | 127 |
| K29 | D | 27 | 59 | 91 | 123 |
| K30 | E | 18 | 50 | 82 | 114 |
| K31 | F | 32 | 64 | 96 | 128 |
| K32 | G | 23 | 55 | 87 | 119 |

 

Collegamento del sensore di ricezione del telecomando

- Il sensore di ricezione a infrarossi si collega alla porta AD7 dei controller MR-C3000 e MR-C3024.

 

Come impostare l'ID del telecomando

- Premere contemporaneamente per circa 2 secondi il pulsante P1 e il tasto con il numero di ID che si vuole impostare (uno tra 1, 2, 3, 4).

 

Esempio di programma RoboBASIC)

 

*'\*\*\*\* Esempio di programma MR-IR3 \*\*\*\**

DIM IR\_ID\_CODE
AS BYTE

CONST IR\_ID = 1

 

*‘Parte di controllo dell'ID*

IF IR\_ID = 1
THEN

IR\_ID\_CODE = 0

ELSEIF IR\_ID =
2 THEN

IR\_ID\_CODE = 32

ELSEIF IR\_ID =
3 THEN

IR\_ID\_CODE = 64

ELSEIF IR\_ID =
4 THEN

IR\_ID\_CODE = 96

ENDIF

 

MAIN:

A = REMOCON(1)

A = A - IR\_ID\_CODE

 

             ON
A GOTO MAIN, KEY1, KEY2, KEY3, KEY4,
……

GOTO MAIN

END

 

KEY1:

             ………

             GOTO
MAIN

 

KEY2:

             ………

             GOTO
MAIN

 

KEY3:

             ………

             GOTO
MAIN
