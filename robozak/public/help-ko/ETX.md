**ETX           데이타송신**

ETX포트를 이용하여 RS232 신호를 송신한다. 3000

![](ETX.files/image001.gif)

             -
영문구문 : ETX *[포트속도], [데이터]*

             -
한글구문 : 데이타송신 *[포트속도], [데이터]*

![](ETX.files/image002.gif)

MR-C3000 계열 컨트롤러의 ETX 포트를 이용하여 RS232방식의 데이터를 송신한다. *[포트속도]* 의 의미는
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

 

*[데이터]*는 송신포트를 통해 보내는 데이터 값이다. *[데이터]*는
숫자, 상수, 변수 등을 사용할 수 있다. 만약 "A"문자를 송신하려고 하면, "A"에 해당하는 아스키 코드를
보내야 하는데, 다음과 같이 하면 간단히 송신할 수 있다.

 

DIM I AS BYTE

 

I = "A"

ETX  9600, I

 

 즉, 문자열을 변수에
대입하게 되면 문자열 중 맨 첫 글자의 아스키 코드 값이 변수에 대입된다.

 *MR-C2000
계열 컨트롤러에서 RS232 신호 송신을 위해서는 ETX(데이타송신) 명령어 대신 TX(송신) 명령어를 사용한다.*
