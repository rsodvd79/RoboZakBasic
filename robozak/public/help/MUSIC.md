**MUSIC       Musica**

Esegue una musica sul piezo. 3000

![](MUSIC.files/image001.gif)

             -
Sintassi inglese : MUSIC "*[stringa musicale]*"

             -
Sintassi coreana : 음악 "*[stringa musicale]*"

 

![](MUSIC.files/image002.gif)

roboBASIC e il controller Mini Robot offrono la funzione di eseguire musica. In *[stringa musicale]* si inseriscono i dati di esecuzione; i caratteri utilizzabili nella stringa e il loro significato sono i seguenti.

 

|  |  |  |
| --- | --- | --- |
| **Stringa** | | **Descrizione** |
| **Inglese/simboli** | **Coreano** |
| C | 도 | Nota “Do” |
| D | 레 | Nota “Re” |
| E | 미 | Nota “Mi” |
| F | 파 | Nota “Fa” |
| G | 솔 | Nota “Sol” |
| A | 라 | Nota “La” |
| B | 시 | Nota “Si” |
| [ |  | Accorcia la durata della nota di 1,5 volte, per le terzine |
| ] |  | Allunga la durata della nota di 1,5 volte, per le terzine |
| O | 옥 | Seleziona l'ottava. |
| M | 중 | Seleziona la terza ottava. |
| . |  | Allunga la nota di 1,5 volte. |
| #, + |  | Alza di un semitono (diesis, #). |
| $, - |  | Abbassa di un semitono (bemolle, ♭). |
| P, ,(virgola) | 쉼 | Pausa: non emette suono per una certa durata. |
| <, L |  | Scende di un'ottava. |
| >, H |  | Sale di un'ottava. |

 

I controller della serie MR-C3000 gestiscono 7 livelli di ottava (Octave); la durata della nota si indica con un numero da 0 a 9, il cui significato è il seguente. Per le note puntate si può usare il numero oppure indicarle con “.” nella *[stringa musicale]*.

![](MUSIC.files/image003.gif)

 

*Nei controller della serie MR-C2000 si usa il comando PLAY al posto del comando MUSIC.*

 I controller della serie MR-C3000 dispongono di un comando separato, TEMPO, per il tempo (Tempo): consultare tale comando.

 

![](MUSIC.files/image004.gif)

MUSIC “O34GGAA GGE GGEED”

MUSIC “O3GGA4.A GGE GEDEC”
