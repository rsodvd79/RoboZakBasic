**AIMOTORIN()   AI모터입력()**

AI모터의 현재 값을 읽어들인다. 3000

![](AIMOTORIN.files/image001.gif)

             -
영문구문 : AIMOTORIN (*[모터번호]*)

             -
한글구문 : AI모터입력 (*[모터번호]*)

 

![](AIMOTORIN.files/image002.gif)

MR-C3000 계열 컨트롤러에 연결된 AI모터의 현재 위치 값을 입력 받아, 프로그램으로 쉽게 제어할 수 있게 한다.

*[모터번호]*는 0-30까지의 숫자, 상수가 가능하며, 서보모터가 연결된 경우
10-190의 모터 각도를 읽어 들이게 되고, 서보모터가 연결되어 있지 않는 경우에는 0 값을 읽어 들이게 된다.

 

![](AIMOTORIN.files/image003.gif)

DIM  AI5  AS  BYTE

 

AIMOTOR INIT

AIMOTOR SETON

AIMOTOR 5                                *‘5번
AI모터 사용설정*

  AI5 =
AIMOTORIN(5)                   *‘5번
AI모터의 값을 AI5 변수에 저장한다.*
