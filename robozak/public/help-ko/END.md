**END     끝**

프로그램의
실행을 끝낸다.

![](END.files/image001.gif)

 

             -
영문구문 : END

             -
한글구문 : 끝

 

![](END.files/image002.gif)

 

미니로봇컨트롤러는 전원을
인가하면 2초 후에 EEPROM에 저장된 프로그램의 수행을 바로 시작한다. 일반적인 컨트롤러 프로그램의 경우 계속
전원을 인가하는 동안에는 무한히 프로그램의 수행하는 구조로 만들어진다. 그러므로 대부분의 경우 외부의 입력 조건에 따라 조건 수행하는 프로그램으로
구성되어 있지만, 프로그램의 중간에 또는 프로그램의 실행문 다음에 서브루틴 작성 등의 구조적인 프로그래밍을 할 경우에는 프로그램의 수행 끝을 알리는
END(끝)명령을 사용한다.

 

![](END.files/image003.gif)

 

① 프로그램 수행도중 프로그램의 수행을 끝낸다.

 

DIM A AS BYTE

 

START: A =
IN(0)

IF A = 1 THEN END

...........

GOTO START

 

② 구조적인 서브루틴을 작성할 수 있다.

 

DIM A AS BYTE

 

A = BYTEIN(0)

IF A = 1 THEN

   GOSUB L1

ELSEIF A = 3 THEN

   GOSUB L2

ELSEIF A = 4 THEN

   GOSUB L3

ELSE

   GOSUB L4

ENDIF

...........

END

 

L1:
..............

RETURN

 

L2:
.............

RETURN

 

L3:
............

RETURN

 

L4: ...........

RETURN
