**SERVO       서보**

서보 모터를 1개 동작한다.

![](SERVO.files/image001.gif)

             -
영문구문 : SERVO *[모터번호],[모터각도]*

             -
한글구문 : 서보 *[모터번호],[모터각도]*

 

![](SERVO.files/image002.gif)

서보모터를 원하는 각도로
동작시킨다. MR-C2000 계열 컨트롤러의 경우 PWM 기능은 해제된다. *[모터번호]*는 서보모터가
연결된 모터 포트 번호이며, *[모터각도]*는 10-190 사이의 숫자, 상수 또는 바이트 형의 변수 사용이 가능하다.

 

![](SERVO.files/image003.gif)

 

예1)

             SERVO
1,130                                 *'
1번 모터를 130위치(각도 +30도)로 동작*

예2)

             DIM
I AS BYTE

 

             FOR 
I = 10 TO 190

                           SERVO
4, I

                           DELAY
100

             NEXT
I
