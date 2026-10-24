**DIM ... AS       선언
... 는**

Dichiara una variabile.

![](DIM%20AS.files/image001.gif)

① Dichiarazione di una variabile singola :

     - Sintassi inglese : DIM [nome variabile] AS [tipo variabile]

     - Sintassi coreana : 선언 [nome variabile] 는 [tipo variabile]

 

② Dichiarazione di più variabili :

     - Sintassi inglese : DIM [nome variabile] AS [tipo variabile], [nome variabile]
AS [tipo variabile], …..

     - Sintassi coreana : 선언 [nome variabile] 는 [tipo variabile], [nome variabile]
는 [tipo variabile], …..

 

![](DIM%20AS.files/image002.gif)

 Le variabili usate in roboBASIC devono prima essere dichiarate con il comando DIM (선언). La dichiarazione di una variabile con il comando DIM (선언) specifica sempre il tipo della variabile tramite AS (는). Nel nome della variabile da dichiarare le lettere inglesi non distinguono maiuscole e minuscole, e non è possibile dichiarare più volte lo stesso nome di variabile.

 Nei programmi per robot si usano le variabili quando si vuole acquisire il valore di un sensore o il valore di una conversione analogica ed elaborarlo in base al valore. Un uso appropriato delle variabili permette quindi di scrivere programmi efficaci; il numero di variabili utilizzabili è fissato in base al tipo di controller del robot.

 La serie MR-C2000 può usare variabili per un totale di 30 byte, mentre la serie MR-C3000 per un totale di 256 byte. Poiché una variabile di tipo byte occupa 1 byte e una di tipo intero 2 byte, conviene dichiarare il tipo in modo appropriato in base all'ambito d'uso della variabile, così da poter utilizzare il maggior numero possibile di variabili.

 

![](DIM%20AS.files/image003.gif)

             DIM
I AS INTEGER                       *'Dichiara I come intero*

             DIM
J AS BYTE                         *'Dichiara J come byte*

             선언
K 는 정수                        *'Dichiara K come intero*

             선언
모터\_1 는 바이트, 모터\_2 는 바이트

                                                        
       *'Dichiara 모터\_1 e 모터\_2 come byte*
