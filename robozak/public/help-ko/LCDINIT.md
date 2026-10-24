**LCDINIT          화면초기**

LCD모듈을 초기화한다.

 

![](LCDINIT.files/image001.gif)

             -
영문구문 : LCDINIT

             -
한글구문 : 화면초기

 

![](LCDINIT.files/image002.gif)

LCD모듈을 사용하기 전에 항상 LCD모듈을 초기화하여야 한다. 초기화하지 않은 LCD모듈을 이용하여 문자열을 출력하면 원하지
않는 이상한 문자가 나타날 수 있으므로, 사용하기 전 프로그램의 처음 부분에 항상 LCDINIT(화면초기) 명령어를 사용한다.

 LCD모듈을 초기화하면
LCD화면의 모든 글자를 지우고, 커서를 맨 첫 위치로 보낸다.

 

![](LCDINIT.files/image003.gif)

LCDINIT        *'LCD모듈을 초기화한다.*
