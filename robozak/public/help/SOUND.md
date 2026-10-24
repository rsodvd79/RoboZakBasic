**SOUND      Suono**

Genera un tono a una determinata frequenza sul piezo. 2000

![](SOUND.files/image001.gif)

             -
Sintassi inglese : SOUND *[nota], [durata], [nota], [durata], ....*

             -
Sintassi coreana : 소리 *[nota], [durata], [nota], [durata], ....*

 

![](SOUND.files/image002.gif)

Nei controller serie MR-C2000 si possono impostare direttamente sulla porta del piezo il segnale alla frequenza desiderata e il tempo di ritardo. I valori impostabili vanno da 1 a 254 e la frequenza corrispondente a ciascun valore è indicata nella tabella seguente. L'unità della durata è di circa 11msec.

 

|  |  |  |  |  |  |
| --- | --- | --- | --- | --- | --- |
| **Valore di ingresso** | **Frequenza(Hz)** | **Ingresso** | **Frequenza(Hz)** | **Ingresso** | **Frequenza(Hz)** |
| 1 | 38.86k | 70 | 800 | 160 | 389 |
| 2 | 23.81k | 80 | 775 | 170 | 365 |
| 5 | 11.11k | 90 | 689 | 180 | 344 |
| 10 | 5.88k | 100 | 621 | 190 | 327 |
| 20 | 3.00k | 110 | 565 | 200 | 311 |
| 30 | 2.00k | 120 | 518 | 210 | 295 |
| 40 | 1.54k | 130 | 478 | 220 | 283 |
| 50 | 1.23k | 140 | 444 | 230 | 270 |
| 60 | 1.00k | 150 | 413 | 240 | 260 |

 

|  |  |
| --- | --- |
| **Tempo** | **Durata** |
| 0.5 secondi | 45 |
| 1 secondo | 90 |
| 2 secondi | 180 |

 

![](SOUND.files/image003.gif)

 

SOUND 60, 90, 60, 180, 30, 45

*‘Genera una frequenza di 1KHz per 1 secondo e per 2 secondi, poi una frequenza di 2KHz per 0,5 secondi.*
