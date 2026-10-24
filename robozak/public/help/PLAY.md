**PLAY         연주**

Suona una melodia sul piezo. 2000

![](PLAY.files/image001.gif)

             -
Sintassi inglese : PLAY "*[stringa di esecuzione]*"

             -
Sintassi coreana : 연주 "*[stringa di esecuzione]*"

 

![](PLAY.files/image002.gif)

roboBASIC e il mini controller robot offrono la funzione di suonare musica. La *[stringa di esecuzione]* contiene i dati musicali; i caratteri utilizzabili nella stringa e il loro significato sono i seguenti.

 

|  |  |  |
| --- | --- | --- |
| **Stringa** | | **Descrizione** |
| **Lettere inglesi/simboli** | **Coreano** |
| C | 도 | Nota "do" |
| D | 레 | Nota "re" |
| E | 미 | Nota "mi" |
| F | 파 | Nota "fa" |
| G | 솔 | Nota "sol" |
| A | 라 | Nota "la" |
| B | 시 | Nota "si" |
| T | 템포, 박자, 속도 | Regola il tempo (velocità) di esecuzione. |
| L | 저 | Seleziona l'ottava bassa. |
| M | 중 | Seleziona l'ottava media. |
| H | 고 | Seleziona l'ottava alta. |
| #, + |  | Alza di un semitono (#). |
| $, - |  | Abbassa di un semitono (♭). |
| P, ,(virgola) | 쉼 | Pausa di una certa durata senza emettere suono. |
| < |  | Scende di un'ottava. |
| > |  | Sale di un'ottava. |

 

 Il tempo (T) imposta la velocità delle note durante l'esecuzione; il valore predefinito è 7 e si può impostare da 1 a 0. Il valore 0 equivale a 10 e indica la velocità più lenta, mentre il valore 1 indica la velocità più rapida.

 I controller della serie MR-C2000 possono gestire 3 livelli di ottava (Octave), chiamati rispettivamente ottava bassa, media e alta.

 Per la durata della nota si usano i numeri da 0 a 9, il cui significato è il seguente.

![](PLAY.files/image003.gif)

 

Se si omette la durata della nota, si suona con la durata della nota precedente. "4CDEF8G" suona do, re, mi, fa come semiminime e sol come croma. Quando si usa l'istruzione PLAY (연주), per impostazione predefinita si parte dall'ottava media, con semiminima, nota do e tempo 7; il PLAY (연주) successivo eredita l'ottava e il tempo precedenti.

 

① La durata della nota si scrive prima della nota.   [ 4도  8미  6파   ]

② Il diesis o il bemolle si scrivono prima della nota.  [ #도  $미  +파   -솔   4#도 #4도 ]

 

*Nei controller della serie MR-C3000 si usa il comando MUSIC (음악) al posto del comando PLAY (연주).*

  

 

![](PLAY.files/image004.gif)

 

PLAY “M4GGAA GGE GGEED”

PLAY “M4GGAA GGE GEDEC”
