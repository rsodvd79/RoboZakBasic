**RCIN()       조종기입력()**

Legge il valore dell'impulso ricevuto da un trasmettitore e ricevitore RC. 3000

![](RCIN.files/image001.gif)

             -
Sintassi inglese : RCIN (*[porta ricezione RC]*)

             -
Sintassi coreana : 조종기입력 (*[porta ricezione RC]*)

 

![](RCIN.files/image002.gif)

Collegando un trasmettitore e un ricevitore RC alla porta di ingresso AD del controller della serie MR-C3000, si legge il valore ricevuto dal trasmettitore. La tabella seguente mostra la configurazione di *[porta ricezione RC]*.

 

|  |  |
| --- | --- |
| **Controller serie MR-C3000**  **Numero porta AD (numero porta I/O digitale)** | **Porta ricezione RC** |
| Porta 0 (porta 32) | Porta ricezione RC 0 |
| Porta 1 (porta 33) | Porta ricezione RC 1 |
| Porta 2 (porta 34) | Porta ricezione RC 2 |
| Porta 3 (porta 35) | Porta ricezione RC 3 |
| Porta 4 (porta 36) | Porta ricezione RC 4 |
| Porta 5 (porta 37) | Porta ricezione RC 5 |
| Porta 6 (porta 38) | Porta ricezione RC 6 |
| Porta 7 (porta 39) | Porta ricezione RC 7 |

 

 Il trasmettitore RC collegato al controller della serie MR-C3000 deve essere di tipo FM anziché AM, per evitare malfunzionamenti dovuti a interferenze radio; usando un trasmettitore con molte funzioni si può controllare il robot in modi molto più vari.

 

 

 

![](RCIN.files/image003.gif)![](RCIN.files/image004.gif)![](RCIN.files/image005.gif)![](RCIN.files/image006.gif)    
    ![](RCIN.files/image007.gif)

 

![](RCIN.files/image008.gif)

 

DIM
A AS BYTE

 

   A
= RCIN(0)                               *‘Salva nella variabile A il segnale del ricevitore collegato alla porta di ricezione RC 0*
