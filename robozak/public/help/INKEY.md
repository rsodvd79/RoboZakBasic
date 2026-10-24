**INKEY  키입력**

Legge il valore di un tasto (interruttore) dalla porta di ingresso. 2000

![](INKEY.files/image001.gif)

             -
Sintassi inglese : INKEY (*[numero porta]*)

             -
Sintassi coreana : 키입력 (*[numero porta]*)

 

![](INKEY.files/image002.gif)

Quando si usa un tasto, come un pulsante, tramite la porta di ingresso, l'utente crede di premerlo una sola volta, ma elettricamente/meccanicamente l'interruttore viene premuto innumerevoli volte. Questo fenomeno si chiama rimbalzo (chattering) e può causare la lettura di valori errati, per cui serve un circuito aggiuntivo anti-rimbalzo. I controller miniROBOT implementano l'anti-rimbalzo via software, senza circuiti aggiuntivi; per usarlo basta applicare il comando INKEY(키입력) alla porta di ingresso.

 

![](INKEY.files/image003.gif)

Salva nella variabile A lo stato di pressione del tasto (interruttore) collegato alla porta 0.

 

DIM A AS BYTE

 

A = INKEY(0)
