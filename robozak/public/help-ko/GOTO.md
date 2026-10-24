**GOTO        가기**

지정한
위치로 무조건 이동한다.

![](GOTO.files/image001.gif)

             -
영문구문 : GOTO *[행라벨]*

             -
한글구문 : 가기 *[행라벨]*

 

![](GOTO.files/image002.gif)

GOTO (가기) 명령은 무조건 라벨이 지정된 행으로 분기하며, 행라벨은 프로그램 안에 반드시 선언되어 있어야 한다.
GOTO 명령문을 너무 많이 사용하면 프로그램을 이해하기 어려우므로, 가능한 복잡하지 않은 범위 내에서 사용하도록 하며, 되도록 구조화된 흐름제어명령문(IF...THEN문,
FOR...NEXT문)을 사용하도록 한다.

 

![](GOTO.files/image003.gif)

DIM I AS INTEGER

DIM J AS BYTE

 

I = 7

IF I = 6 THEN GOTO L1

  ................

L1:  J = 1

OUT I, J
