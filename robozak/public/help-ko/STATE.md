**STATE()          출력상태()**

출력포트의
현재 출력상태 값을 읽어온다.

![](STATE.files/image001.gif)

             -
영문구문 : STATE (*[포트번호]*)

             -
한글구문 : 출력상태 (*[포트번호]*)

 

![](STATE.files/image002.gif)

출력포트를 통해서 신호를
내보낸 후 그 포트의 값의 상태를 알려고 할 때는 다시 입력(IN)함수를 사용해서는 안되고, 출력포트의 상태를 알아내는
출력상태(STATE)함수를 사용하여야 한다.

 

![](STATE.files/image003.gif)

 미니로봇 컨트롤러의
1번 포트의 출력상태를 알아보는 프로그램의 예이다.

 

DIM A AS BYTE

 

OUT 1, 1

A = STATE(1)*'
A = 1*

OUT 1, 0

A = STATE(1)*'
A = 0*
