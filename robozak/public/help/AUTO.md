**AUTO        Automatico**

Passa al programma modello (template).

![](AUTO.files/image001.gif)

             -
Sintassi inglese : GOTO AUTO

             -
Sintassi coreana : 가기 자동

![](AUTO.files/image002.gif)

Comando per eseguire il programma di movimento dell'area di performance di base predefinita (area template).

![](AUTO.files/image003.gif)

GOTO AUTO                    *‘
Salta al programma template.*

 

*Nota) Questo comando è dedicato all'MR-C3024 e ai robot Robonova.*

 

  

 

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
| ACTION | tasto | Contenuto del movimento | Variabile | Codice |
| 0 | power | ON : motori accesi -> posizione base  OFF: posizione seduta -> motori spenti | A16 | 16 |
| 1 | 1 | Saluto -> posizione base |  | 1 |
| 2 | 2 | Alza le braccia -> posizione base |  | 2 |
| 3 | 3 | Siediti -> posizione base |  | 3 |
| 4 | 4 | Siediti -> braccia in alto -> posizione base |  | 4 |
| 5 | 5 | Movimento di sollevamento di una gamba -> posizione base |  | 5 |
| 6 | 6 | Apre i piedi -> apre le braccia a destra e sinistra -> inclinazione laterale -> posizione base |  | 6 |
| 7 | 7 | Battito d'ali |  | 7 |
| 8 | 8 | Calcio con entrambi i piedi |  | 8 |
| 9 | 9 | Verticale sulle mani |  | 9 |
| 10 | 0 | Corsa veloce |  | 10 |
| 11 | \* | Svolta a sinistra |  | 22 |
| 12 | # | Svolta a destra |  | 24 |
| 13 |  | Avanti |  | 11 |
| 14 |  | A sinistra |  | 14 |
| 15 |  | Siediti<->alzati | A26 | 26 |
| 16 |  | A destra |  | 13 |
| 17 |  | Indietro |  | 12 |
| 18 |  | Capriola in avanti |  | 21 |
| 19 |  | Capriola a sinistra |  | 28 |
| 20 |  | Attacco frontale |  | 29 |
| 21 |  | Capriola a destra |  | 30 |
| 22 |  | Capriola all'indietro |  | 31 |
| 23 | A | Attacco a sinistra |  | 15 |
| 24 | B | Attacco a destra |  | 20 |
| 25 | C | Affondo anteriore sinistro |  | 17 |
| 26 | D | Affondo anteriore destro |  | 27 |
| 30 | P1 | Rialzarsi (davanti) |  | 25 |
| 31 | P2 | Rialzarsi (dietro) |  | 19 |
|  |  |  |  |  |
|  |  |  |  |  |
|  | E,F,G | 27,28,29    riservati  18,32,23 |  | 18 |
|  |  |  |  |  |
|  |  |  |  |  |
