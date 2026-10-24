**BYTEIN()    Ingresso byte()**

Legge un segnale da una porta di ingresso a byte.

![](BYTEIN.files/image001.gif)

             -
Sintassi inglese : BYTEIN(*[numero porta byte]*)

             -
Sintassi coreana : 바이트입력(*[numero porta byte]*)

 

![](BYTEIN.files/image002.gif)

Le porte del mini controller robot possono ricevere e inviare segnali un bit alla volta, come nei comandi IN/OUT, ma in alcuni casi occorre farlo per unità (qui, porte byte da 8 porte). Il mini controller robot imposta per ciascuna porta le seguenti porte byte.

 

Controller della serie MR-C2000)

 

|  |  |
| --- | --- |
| **Porta byte** | **Porte** |
| 0 | Porte da 0 a 7 (la porta 0 è il bit meno significativo della porta byte) |
| 1 | Porte da 8 a 11 (la porta 8 è il bit meno significativo della porta byte) |

 

Controller della serie MR-C3000)

 

|  |  |
| --- | --- |
| **Porta byte** | **Porte** |
| 0 | Porte da 0 a 7 (la porta 0 è il bit meno significativo della porta byte) |
| 1 | Porte da 8 a 15 (la porta 8 è il bit meno significativo della porta byte) |
| 2 | Porte da 16 a 23 (la porta 16 è il bit meno significativo della porta byte) |
| 3 | Porte da 24 a 31 (la porta 24 è il bit meno significativo della porta byte) |
| 4 | Porte da 32 a 39 (la porta 32 è il bit meno significativo della porta byte) |

 

Il comando BYTEIN riceve un segnale tramite la porta di ingresso a byte del mini controller robot e ne memorizza il valore in una variabile. La variabile deve essere dichiarata di tipo byte o intero.

 

![](BYTEIN.files/image003.gif)

 A
= BYTEIN(0)*'Tutti i segnali dalla porta 0 alla porta 7
vengono*

*‘**inseriti nella variabile A.*
