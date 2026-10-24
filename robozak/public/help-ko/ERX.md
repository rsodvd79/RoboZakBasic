**ERX           데이타수신**

ERX포트를 이용하여 RS232 신호를 수신한다. 3000

![](ERX.files/image001.gif)

             -
영문구문 : ERX *[포트속도], [수신변수], [수신에러처리라벨]*

             -
한글구문 : 데이타수신 *[포트속도], [수신변수], [수신에러처리라벨]*

 

![](ERX.files/image002.gif)

MR-C3000 계열 컨트롤러의 ERX 포트를 이용하여 RS232방식의 데이터를 수신한다. *[포트속도]* 의 의미는
다음과 같다.

 

|  |  |
| --- | --- |
| **숫자** | **포트 설정** |
| 2400 | 2400bps, 8Bit data, No parity, 1 Stop bit |
| 4800 | 4800bps, 8Bit data, No parity, 1 Stop bit |
| 9600 | 9600bps, 8Bit data, No parity, 1 Stop bit |
| 14400 | 14400bps, 8Bit data, No parity, 1 Stop bit |
| 19200 | 19200bps, 8Bit data, No parity, 1 Stop bit |
| 28800 | 28800bps, 8Bit data, No parity, 1 Stop bit |
| 38400 | 38400bps, 8Bit data, No parity, 1 Stop bit |
| 57600 | 57600bps, 8Bit data, No parity, 1 Stop bit |
| 76800 | 76800bps, 8Bit data, No parity, 1 Stop bit |
| 115200 | 115200bps, 8Bit data, No parity, 1 Stop bit |
| 230400 | 230400bps, 8Bit data, No parity, 1 Stop bit |

 

 *[수신변수]*는
수신된 데이터를 저장할 변수를 말하며, 반드시 선언된 바이트형 변수를 사용한다.

 *[수신에러처리라벨]*은
데이터가 수신되지 않았으면(통신버퍼가 비어있으면), 처리하는 부분이 시작되는 위치의 라벨을 말한다. RS232포트를 통해서 데이터가 수신될 때까지
기다리는 프로그램은 다음과 같이 작성할 수 있다.

 

Retry:

             ERX
 9600,  A,  Retry

 

 *MR-C2000
계열 컨트롤러에서 RS232 신호 수신을 위해서는 ERX(데이타수신) 명령어 대신 RX(수신) 명령어를 사용한다.*
