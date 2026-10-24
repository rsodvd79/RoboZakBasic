**‘$DEVICE         ‘$컨트롤러**

Imposta il controller a cui si applica il programma in scrittura.

![](DEVICE.files/image001.gif)

             -
Sintassi inglese : ‘$DEVICE *[tipo di controller]*

             -
Sintassi coreana : ‘$컨트롤러 *[tipo di controller]*

![](DEVICE.files/image002.gif)

Quando si scrive un programma con roboBASIC, serve a indicare direttamente nel programma il controller su cui verrà eseguito. Attualmente come *[tipo di controller]* si possono usare "MRC2000" e "MRC3000".

 Compilando il programma, il "tipo di controller" corrente nel menu di roboBASIC viene cambiato automaticamente in quello indicato nel comando DEVICE.

 

![](DEVICE.files/image003.gif)

'$DEVICE 
MRC2000        *'Il programma in scrittura è destinato*

*‘**ai controller della serie MR-C2000.*
