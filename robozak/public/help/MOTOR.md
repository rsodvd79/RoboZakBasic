**MOTOR      Impostazione servomotore**

Imposta l'uso dei servomotori.

![](MOTOR.files/image001.gif)

**Controller della serie MR-C2000)**

 

             -
Sintassi inglese : MOTOR *[numero servomotore]*

             -
Sintassi coreana : 모터설정 *[numero servomotore]*

 

**Controller della serie MR-C3000)**

 

             -
Sintassi inglese : MOTOR *[numero servomotore] / [gruppo]*

             -
Sintassi coreana : 모터설정 *[numero servomotore] / [gruppo]*

 

![](MOTOR.files/image002.gif)

**Attivazione nei controller della serie MR-C2000)**

 

Nella serie MR-C2000 si possono usare 6 servomotori, dalla porta 0 alla porta 5 del mini controller robot, e *[numero servomotore]* può andare dal servomotore 0 al servomotore 5.

 *Se il numero del servomotore è 6, l'uso viene attivato per tutti i servomotori da 1 a 6.* Quando si collegano servomotori al mini controller robot per usarli, occorre sempre attivarne l'uso all'inizio del programma; prestare sempre attenzione, perché *eseguendo comandi relativi al movimento dei servomotori senza averne attivato l'uso, i servomotori non si muovono affatto.* Per *[numero servomotore]* si possono usare solo numeri o costanti da 0 a 6.

 

 Esempio 1) MOTOR
6                              *‘Attiva l'uso di tutti i
servomotori (0-5)*

 Esempio 2) MOTOR
2                              *‘Attiva l'uso del
servomotore 2*

 

 

**Attivazione nei controller della serie MR-C3000)**

Nella serie MR-C3000 si possono usare in totale 32 servomotori, dalla porta 0 alla porta 31 del mini controller robot; l'uso si può attivare indicando un servomotore alla volta con *[numero servomotore]* oppure tutti insieme con *[gruppo]*.

Poiché *[numero servomotore]* può essere un numero, una costante o una variabile di tipo byte, sono possibili diversi modi di impostazione.

 

![](MOTOR.files/image003.gif)

Esempio 1) MOTOR 0              *'Attiva l'uso del
servomotore 0.*

Esempio 2) MOTOR
G6A             *'Attiva l'uso del
gruppo di servomotori 6A (0~5).*

    
MOTOR G6C                   *'Attiva l'uso del
gruppo di servomotori 6C (12~17).*

Esempio 3) MOTOR G8A                       *'Attiva l'uso del
gruppo di servomotori 8A (0~7).*

Esempio 4) Attivazione dei servomotori con una variabile

    DIM I AS
BYTE

    FOR I = 0
TO 31                   *'Usando la variabile
I attiva i servomotori da 0 a 31*

                             
*'.*

        
MOTOR I

    NEXT I

Esempio 5) MOTOR G24                        *'Attiva l'uso del
gruppo di servomotori 24 (0~23).*

Esempio 6) MOTOR ALLON                   *'Attiva l'uso di tutti i
servomotori (0~31).*
