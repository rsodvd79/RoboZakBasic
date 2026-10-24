**MINIOUT         미니출력**

Trasmette un segnale minibus tramite la porta minibus. 2000

![](MINIOUT.files/image001.gif)

             -
Sintassi inglese : MINIOUT *[dati] , [dati], ......*

             -
Sintassi coreana : 미니출력 *[dati] , [dati], ......*

 

![](MINIOUT.files/image002.gif)

Trasmette dati in modalità minibus tramite la porta minibus (porta 6) dei controller della serie MR-C2000. Il minibus svolge la stessa funzione di una porta RS232 con impostazioni "4800bps, 8Bit data, No parity, 1 Stop bit". *[dati]* può essere un numero, una costante, una variabile, ecc., e si può inviare un numero illimitato di dati. Attenzione però: *non è possibile inviare il numero 0*.

 

![](MINIOUT.files/image003.gif)

MINIOUT 100, 20, 76, 65
