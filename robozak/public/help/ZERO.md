**ZERO        영점**

Imposta l'angolo di riferimento (punto zero) dei servomotori.

![](ZERO.files/image001.gif)

**Controller della serie MR-C2000)**

 

- Sintassi inglese : ZERO *[angolo di riferimento motore 0], [angolo di riferimento motore 1], ..., [angolo di riferimento motore 5]*

- Sintassi coreana : 영점 *[angolo di riferimento motore 0], [angolo di riferimento motore 1], ..., [angolo di riferimento motore 5]*

 

**Controller della serie MR-C3000)**

 

- Sintassi inglese : ZERO *[gruppo], [angolo di riferimento motore n], .....*

- Sintassi coreana : 영점 *[gruppo], [angolo di riferimento motore n], .....*

 

![](ZERO.files/image002.gif)

L'angolo centrale corrispondente all'angolo di riferimento 0° del servomotore, ovvero il punto zero (Zero Point) corrispondente all'angolo motore 100, presenta errori per ogni servomotore a causa della posizione di innesto degli ingranaggi, delle tolleranze di produzione e di altri fattori. Ad esempio, il punto zero di un servomotore può essere 99 o 98, oppure 101 o 102.

Questo errore del punto di riferimento rende difficile individuare l'esatta posizione originale quando si aziona il servomotore o durante lo smontaggio e il rimontaggio. Per correggerlo si usa il comando ZERO(영점). Se si imposta in anticipo il punto zero di ciascun servomotore prima di usarlo, con il comando MOVE(동작) il movimento avviene prendendo come riferimento l'angolo zero impostato.

 

 

**Uso del punto zero nei controller della serie MR-C2000)**

L'impostazione del punto zero va fatta una sola volta all'inizio; una volta eseguito il comando, il valore di riferimento viene salvato nella EEPROM e si mantiene anche dopo lo spegnimento. Pertanto, per impostare un nuovo punto zero occorre seguire i passaggi seguenti. Le versioni più recenti di RoboBASIC hanno un apposito menu “Imposta punto zero” (영점설정) che rende superflua questa procedura.

 

 1) Impostare tutti i punti zero esistenti a 100, cioè eliminare tutti i valori impostati in precedenza.

 2) Impostare la direzione di tutti i motori in avanti (normale).

 3) Spegnere (OFF) e riaccendere (ON) l'alimentazione.

 4) Azionare il motore con la funzione online e spostarlo nella posizione ritenuta centrale.

 5) Memorizzare/annotare il valore visualizzato del motore così spostato e inserirlo con il comando di punto zero.

 

Esempio 1)  Eliminare i valori di punto zero esistenti.

 

ZERO
100,100,100,100,100,100

DIR  
1,1,1,1,1,1                         *'Imposta la direzione dei motori in avanti*     

 

Esempio 2) Reimpostare il punto zero.  Annotare il valore della posizione centrale di ciascun motore.

 

DIR 
1,1,1,1,1,1

ZERO 100, 101,
99              
   *'Imposta il punto zero di 3 servomotori*

ZERO 102, 100,
100, 99, 101, 100  *'Imposta il punto zero di 6 servomotori*

 

Nei controller della serie MR-C2000 l'angolo di impostazione del punto zero deve essere un valore compreso tra 90 e 110.

 

**Uso del punto zero nei controller della serie MR-C3000)**

 Per impostare il punto zero su MR-C3000 è obbligatorio dichiarare prima il gruppo.

 

 ZERO G8B, 80, 120, 115,
80, 117, 88, 95, 120

                                        *‘**Imposta il punto zero di ciascun servomotore del gruppo 8B (servomotori dal n. 8 al n. 15).*

 

 Nei controller della serie MR-C3000 il punto zero può essere impostato tra 80 e 120.
