**MINIIN       미니입력**

미니버스
포트를 이용하여 미니버스신호를 수신한다. 2000

![](MINIIN.files/image001.gif)

             -
영문구문 : MINIIN

             -
한글구문 : 미니입력

 

![](MINIIN.files/image002.gif)

MR-C2000 계열 컨트롤러의 미니버스 포트(6번 포트)를 이용하여 미니버스방식의 데이터를 수신한다. 만약 수신된 데이터가
없으면 0값이 들어온다.

 미니버스를 통하여
데이터가 들어올 때까지 대기하는 프로그램은 다음과 같이 작성하면 된다.

 

DIM A AS BYTE

 

Retry:

A = MINIIN

IF A = 0 THEN GOTO Retry
