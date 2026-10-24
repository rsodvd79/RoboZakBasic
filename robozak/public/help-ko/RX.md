**RX       수신**

RX포트를 이용하여 RS232 신호를 수신한다. 2000

![](RX.files/image001.gif)

             -
영문구문 : RX *[포트속도], [수신변수], [수신에러처리라벨]*

             -
한글구문 : 수신 *[포트속도], [수신변수], [수신에러처리라벨]*

 

![](RX.files/image002.gif)

MR-C2000 계열 컨트롤러의 9번 포트를 이용하여 RS232방식의 데이터를 수신한다. *[포트속도]*는 1～4까지의
숫자나 상수를 사용하며, *[포트속도]*의 의미는 다음과 같다.

 

|  |  |
| --- | --- |
| **숫자** | **포트 설정** |
| 1 | 1200bps, 8Bit data, No parity, 1 Stop bit |
| 2 | 2400bps, 8Bit data, No parity, 1 Stop bit |
| 3 | 2400bps, 8Bit data, No parity, 1 Stop bit |
| 4 | 4800bps, 8Bit data, No parity, 1 Stop bit |

 

 *[수신변수]*는
수신된 데이터를 저장할 변수를 말하며, 반드시 선언된 바이트형 변수를 사용한다.

 *[수신에러처리라벨]*은
데이터가 수신되지 않았으면(통신버퍼가 비어있으면), 처리하는 부분이 시작되는 위치의 라벨을 말한다. RS232포트를 통해서 데이터가 수신될 때까지
기다리는 프로그램은 다음과 같이 작성할 수 있다.

 

Retry:

             RX
 4,  A,  Retry

 *MR-C3000
계열 컨트롤러에서 RS232 신호 수신을 위해서는 RX(수신) 명령어 대신 ERX(데이타수신) 명령어를 사용한다.*

 

![](RX.files/image003.gif)

 다음 예는 외부에
연결된 RS232 단자를 통해 아스키 코드 &h80(16진수) 값이 들어오면 0번 포트의 LED를 켜고, 그 외에는 LED를 끄는 프로그램
예이다.

 

             DIM 
A  AS  BYTE

 

Retry:   RX 4, A, Retry

             IF
A = &h80 THEN

                           OUT
0, 0

             ELSE

                           OUT
0, 1

             ENDIF

             GOTO
Retry
