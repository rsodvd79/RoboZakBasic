**PWM         펄스폭**

Aziona un motore DC controllandone la velocità.

![](PWM.files/image001.gif)

             -
Sintassi inglese : PWM *[numero motore],[valore larghezza impulso]*

             -
Sintassi coreana : 펄스폭 *[numero motore],[valore larghezza impulso]*

 

![](PWM.files/image002.gif)

**Controllo PWM nei controller della serie MR-C2000**

I controller della serie MR-C2000 usano le stesse porte per il controllo dei servomotori e per il PWM. Pertanto *[numero motore]* va da 0 a 5. Il comando PWM emette sulla porta del motore un segnale PWM con il valore di larghezza d'impulso indicato. In questo caso la funzione servo viene disattivata.
( [Attenzione] I comandi SERVO/MOVE e il comando PWM non possono essere usati insieme nello stesso programma.)

 

|  |  |  |  |
| --- | --- | --- | --- |
| **Duty** | **Valore larghezza impulso** | **Duty** | **Valore larghezza impulso** |
| 0 | 0 | 60 | 153 |
| 10 | 25 | 70 | 178 |
| 20 | 51 | 80 | 204 |
| 30 | 77 | 90 | 229 |
| 40 | 102 | 100 | 254 |
| 50 | 127 |  |  |

 

             PWM
3, 127                      *'
Uscita PWM con duty del 50% sulla porta motore 3*

 

**Controllo PWM nei controller della serie MR-C3000**

Nei controller della serie MR-C3000 le porte per i servomotori e le porte PWM sono distinte. La serie MR-C3000 dispone di 3 porte PWM in tutto, numerate da 0 a 2. Pertanto *[numero motore]* va da 0 a 2. La frequenza del PWM della serie MR-C3000 è 454Hz.

 

PWM 0, 120                      *'Genera impulsi con duty 120 sulla porta PWM 0.*
