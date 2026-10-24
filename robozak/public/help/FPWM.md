**FPWM        주파수펄스**

Emette un segnale PWM a frequenza variabile. 3000

![](FPWM.files/image001.gif)

             -
Sintassi inglese : FPWM *[porta], [frequenza], [duty]*

             -
Sintassi coreana : 주파수펄스 *[porta], [frequenza], [duty]*

 

![](FPWM.files/image002.gif)

Nei controller della serie MR-C3000 genera impulsi variando la frequenza del PWM. La porta va da 0 a 2, la frequenza da 1 (bassa) a 5 (alta) e il duty da 0 a 255.

 

![](FPWM.files/image003.gif)

             FPWM
0, 1, 127                 *‘Sulla porta PWM 0 del controller serie MR-C3000*

                                                     *‘emette un segnale PWM a bassa frequenza con duty del 50% (127)*
