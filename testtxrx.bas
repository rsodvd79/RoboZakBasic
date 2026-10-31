'================================================
'
' Fermo.bas
'
'================================================

'== auto_main ===================================
'GOTO AUTO
'FILL 255,10000

'=== variabili ==================================
DIM CM AS BYTE ' variabile comando
DIM BF AS BYTE ' varibile buffer
DIM LRV AS BYTE ' varabile loop read buffer
DIM V1 AS BYTE ' variabile temporanea 1
'DIM V2 AS BYTE ' variaible temporanea 2
DIM M01 AS BYTE
DIM M02 AS BYTE
DIM M03 AS BYTE
DIM M04 AS BYTE
DIM M05 AS BYTE
DIM M06 AS BYTE
DIM M07 AS BYTE
DIM M08 AS BYTE
DIM M09 AS BYTE
DIM M10 AS BYTE
DIM M11 AS BYTE
DIM M12 AS BYTE
DIM M13 AS BYTE
DIM M14 AS BYTE
DIM M15 AS BYTE
DIM M16 AS BYTE
DIM M17 AS BYTE
DIM M18 AS BYTE
DIM M19 AS BYTE
DIM M20 AS BYTE
DIM M21 AS BYTE
DIM M22 AS BYTE
DIM M23 AS BYTE
DIM M24 AS BYTE

'=== init =======================================
CM = 0
BF = 0
LRV = 0
V1 = 0
'V2 = 0

PTP SETON
PTP ALLON

'== motor diretion setting ======================
DIR G6A, 1, 0, 0, 1, 0, 0
DIR G6B, 1, 1, 1, 1, 1, 1
DIR G6C, 0, 0, 0, 0, 0, 0
DIR G6D, 0, 1, 1, 0, 1, 0

'== motor start position read ===================
GETMOTORSET G24,1,1,1,1,1,0,1,1,1,0,0,0,1,1,1,0,0,0,1,1,1,1,1,0

'== motor power on  =============================
SPEED 5
MOTOR G24
GOSUB standard_pose
WAIT
TEMPO 230
MUSIC "CDE"

'=== main loop ==================================
MAIN:

    GOSUB READ_VALUE
    CM = BF

    IF CM = 1 THEN ' SET MOTORE
        GOSUB READ_VALUE
        V1 = BF

        GOSUB READ_VALUE

        IF V1=1 THEN
            M01=BF
        ELSEIF V1=2 THEN
            M02=BF
        ELSEIF V1=3 THEN
            M03=BF
        ELSEIF V1=4 THEN
            M04=BF
        ELSEIF V1=5 THEN
            M05=BF
        ELSEIF V1=6 THEN
            M06=BF
        ELSEIF V1=7 THEN
            M07=BF
        ELSEIF V1=8 THEN
            M08=BF
        ELSEIF V1=9 THEN
            M09=BF
        ELSEIF V1=10 THEN
            M10=BF
        ELSEIF V1=11 THEN
            M11=BF
        ELSEIF V1=12 THEN
            M12=BF
        ELSEIF V1=13 THEN
            M13=BF
        ELSEIF V1=14 THEN
            M14=BF
        ELSEIF V1=15 THEN
            M15=BF
        ELSEIF V1=16 THEN
            M16=BF
        ELSEIF V1=17 THEN
            M17=BF
        ELSEIF V1=18 THEN
            M18=BF
        ELSEIF V1=19 THEN
            M19=BF
        ELSEIF V1=20 THEN
            M20=BF
        ELSEIF V1=21 THEN
            M21=BF
        ELSEIF V1=22 THEN
            M22=BF
        ELSEIF V1=23 THEN
            M23=BF
        ELSEIF V1=24 THEN
            M24=BF
        ENDIF

        GOSUB comando_end

    ELSEIF CM = 2 THEN ' WAIT
        WAIT
        GOSUB comando_end

    ELSEIF CM = 3 THEN ' SPEED
        GOSUB READ_VALUE
        SPEED BF
        GOSUB comando_end

    ELSEIF CM = 4 THEN ' OUT
        GOSUB READ_VALUE
        V1 = BF
        GOSUB READ_VALUE
        OUT V1, BF
        GOSUB comando_end

    ELSEIF CM = 5 THEN ' SERVO 2
        SERVO 1, M01
        SERVO 2, M02
        SERVO 3, M03
        SERVO 4, M04
        SERVO 5, M05
        SERVO 6, M06
        SERVO 7, M07
        SERVO 8, M08
        SERVO 9, M09
        SERVO 10, M10
        SERVO 11, M11
        SERVO 12, M12
        SERVO 13, M13
        SERVO 14, M14
        SERVO 15, M15
        SERVO 16, M16
        SERVO 17, M17
        SERVO 18, M18
        SERVO 19, M19
        SERVO 20, M20
        SERVO 21, M21
        SERVO 22, M22
        SERVO 23, M23
        SERVO 24, M24

        MOVE24 ,,,,,,,,,,,,,,,,,,,,,,,
        ' GAMBE
        'MOVE G6A,,,,,,
        'MOVE G6D,,,,,,
        ' BRACCIA
        'MOVE G6B,,,,,,
        'MOVE G6C,,,,,,

        GOSUB comando_end

    ELSEIF CM = 6 THEN ' FERMO
        GOSUB standard_pose
        WAIT
        GOSUB comando_end

    ELSEIF CM = 7 THEN ' AVANTI
        'GOSUB standard_pose
        GOSUB forward_walk
        GOSUB standard_pose
        WAIT
        GOSUB comando_end

    ELSEIF CM = 8 THEN ' SINISTRA
        'GOSUB standard_pose
        GOSUB left_turn
        GOSUB standard_pose
        WAIT
        GOSUB comando_end

    ELSEIF CM = 9 THEN ' DESTRA
        'GOSUB standard_pose
        GOSUB right_turn
        GOSUB standard_pose
        WAIT
        GOSUB comando_end

    ELSEIF CM = 10 THEN ' INDIETRO
        'GOSUB standard_pose
        GOSUB backward_walk
        GOSUB standard_pose
        WAIT
        GOSUB comando_end

    ELSEIF CM = 11 THEN ' SU AVANTI
        GOSUB standard_pose
        GOSUB forward_standup
        GOSUB standard_pose
        WAIT
        GOSUB comando_end

    ELSEIF CM = 12 THEN ' SU DIETRO
        GOSUB standard_pose
        GOSUB backward_standup
        GOSUB standard_pose
        WAIT
        GOSUB comando_end

    ELSEIF CM = 13 THEN ' SERVO
        GOSUB READ_VALUE
        V1 = BF
        GOSUB READ_VALUE
        SERVO V1, BF
        GOSUB comando_end

    ELSEIF CM = 14 THEN ' MOVE
        ' GAMBE
        MOVE G6A,,,,,,
        MOVE G6D,,,,,,
        ' BRACCIA
        MOVE G6B,,,,,,
        MOVE G6C,,,,,,

        GOSUB comando_end
    ENDIF

    GOTO MAIN

READ_VALUE:
    BF = 0
    LRV = 0
    ETX 19200,85

LOOP_READ_VALUE:
    LRV = LRV + 1
    IF LRV > 2 THEN
        BF = 0
        RETURN
    ENDIF
    'DELAY 1

    ERX 19200, BF, LOOP_READ_VALUE

    IF BF = 200 THEN
        BF = 0
    ENDIF

    RETURN

    '================================================

comando_end:
    'ETX 19200,86
    RETURN

standard_pose:
    SPEED 8
    '         001, 002, 003, 004, 005, 006
    'MOVE G6A, 100,  76, 145,  93, 100, 100
    'MOVE G6D, 100,  76, 145,  93, 100, 100
    'MOVE G6B, 100,  30,  80, 175, 100, 100
    'MOVE G6C, 100,  30,  80, 175, 100, 100
    'WAIT
    '       001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 023, 024
    MOVE24  100,  76, 145,  93, 100, 100, 100,  30,  80, 175, 100, 100, 100,  30,  80, 175, 100, 100, 100,  76, 145,  93, 100, 100

    M01 = 100
    M02 = 76
    M03 = 145
    M04 = 93
    M05 = 100
    M06 = 100
    M07 = 100
    M08 = 30
    M09 = 80
    M10 = 175
    M11 = 100
    M12 = 100
    M13 = 100
    M14 = 30
    M15 = 80
    M16 = 175
    M17 = 100
    M18 = 100
    M19 = 100
    M20 = 76
    M21 = 145
    M22 = 93
    M23 = 100
    M24 = 100

    'MOVE24  M01,M02,M03,M04,M05,M06,M07,M08,M09,M10,M11,M12,M13,M14,M15,M16,M17,M18,M19,M20,M21,M22,M23,M24
    WAIT

    RETURN

    '================================================

forward_walk:
    SPEED 5
    '       001, 002, 003, 004, 005, 006, 007, 008, 009, 010, 011, 012, 013, 014, 015, 016, 017, 018, 019, 020, 021, 022, 023, 024
    MOVE24  85,  71, 152,  91, 112,  60, 100,  40,  80,    ,    ,    , 100,  40,  80,    ,    ,    , 112,  76, 145,  93,  92,  60

    SPEED 14
    'left up
    MOVE24  90, 107, 105, 105, 114,  60,  90,  40,  80,    ,    ,    , 100,  40,  80,    ,    ,    , 114,  76, 145,  93,  90,  60
    '---------------------------------------
    'left down
    MOVE24  90,  56, 143, 122, 114,  60,  80,  40,  80,    ,    ,    , 105,  40,  80,    ,    ,    , 113,  80, 145,  90,  90,  60
    MOVE24  90,  46, 163, 112, 114,  60,  80,  40,  80,    ,    ,    , 105,  40,  80,    ,    ,    , 112,  80, 145,  90,  90,  60

    SPEED 10
    'left center
    MOVE24 100,  66, 141, 113, 100, 100,  90,  40,  80,    ,    ,    , 100,  40,  80,    ,    ,    , 100,  83, 156,  80, 100, 100
    MOVE24 113,  78, 142, 105,  90,  60, 100,  40,  80,    ,    ,    , 100,  40,  80,    ,    ,    ,  90, 102, 136,  85, 114,  60

    SPEED 14
    'right up
    MOVE24 113,  76, 145,  93,  90,  60, 100,  40,  80,    ,    ,    ,  90,  40,  80,    ,    ,    ,  90, 107, 105, 105, 114,  60

    'right down
    MOVE24 113,  80, 145,  90,  90,  60, 105,  40,  80,    ,    ,    ,  80,  40,  80,    ,    ,    ,  90,  56, 143, 122, 114,  60
    MOVE24 112,  80, 145,  90,  90,  60, 105,  40,  80,    ,    ,    ,  80,  40,  80,    ,    ,    ,  90,  46, 163, 112, 114,  60

    SPEED 10
    'right center
    MOVE24 100,  83, 156,  80, 100, 100, 100,  40,  80,    ,    ,    ,  90,  40,  80,    ,    ,    , 100,  66, 141, 113, 100, 100
    MOVE24  90, 102, 136,  85, 114,  60, 100,  40,  80,    ,    ,    , 100,  40,  80,    ,    ,    , 113,  78, 142, 105,  90,  60

    SPEED 14
    'left up
    MOVE24  90, 107, 105, 105, 114,  60,  90,  40,  80,    ,    ,    , 100,  40,  80,    ,    ,    , 113,  76, 145,  93,  90,  60
    '---------------------------------------

    SPEED 5
    MOVE24  85,  71, 152,  91, 112,  60, 100,  40,  80,    ,    ,    , 100,  40,  80,    ,    ,    , 112,  76, 145,  93,  92,  60

    RETURN

left_turn:
    SPEED 6
    MOVE G6D,  85,  71, 152,  91, 112,  60
    MOVE G6A, 112,  76, 145,  93,  92,  60
    MOVE G6C, 100,  40,  80,    ,    ,
    MOVE G6B, 100,  40,  80,    ,    ,
    WAIT

    SPEED 9
    MOVE G6A, 113,  75, 145,  97,  93,  60
    MOVE G6D,  90,  50, 157, 115, 112,  60
    MOVE G6B, 105,  40,  70,    ,    ,
    MOVE G6C,  90,  40,  70,    ,    ,
    WAIT

    MOVE G6A, 108,  78, 145,  98,  93,  60
    MOVE G6D,  95,  43, 169, 110, 110,  60
    MOVE G6B, 105,  40,  70,    ,    ,
    MOVE G6C,  80,  40,  70,    ,    ,
    WAIT
    RETURN
    '================================================

right_turn:
    SPEED 6
    MOVE G6A,  85,  71, 152,  91, 112,  60
    MOVE G6D, 112,  76, 145,  93,  92,  60
    MOVE G6B, 100,  40,  80,    ,    ,
    MOVE G6C, 100,  40,  80,    ,    ,
    WAIT

    SPEED 9
    MOVE G6D, 113,  75, 145,  97,  93,  60
    MOVE G6A,  90,  50, 157, 115, 112,  60
    MOVE G6C, 105,  40,  70,    ,    ,
    MOVE G6B,  90,  40,  70,    ,    ,
    WAIT

    MOVE G6D, 108,  78, 145,  98,  93,  60
    MOVE G6A,  95,  43, 169, 110, 110,  60
    MOVE G6C, 105,  40,  70,    ,    ,
    MOVE G6B,  80,  40,  70,    ,    ,
    WAIT
    RETURN

    '================================================
backward_walk:

    SPEED 5
    GOSUB backward_walk1

    SPEED 13
    GOSUB backward_walk2

    SPEED 7

    MOVE G6D,113,  76, 142, 105,  90,  60
    MOVE G6A, 90,  96, 136,  85, 114,  60
    MOVE G6C,100,  40,  80,    ,    ,
    MOVE G6B,100,  40,  80,    ,    ,
    WAIT

    MOVE G6D,100,  62, 146, 108, 100, 100
    MOVE G6A,100,  88, 140,  86, 100, 100
    MOVE G6C, 90,  40,  80,    ,    ,
    MOVE G6B,100,  40,  80,    ,    ,
    WAIT

    MOVE G6D, 90,  56, 143, 122, 114,  60
    MOVE G6A,113,  80, 145,  90,  90,  60
    MOVE G6C, 80,  40,  80,    ,    ,
    MOVE G6B,105,  40,  80,    ,    ,
    WAIT

    SPEED 13

    MOVE G6D, 90, 107, 105, 105, 114,  60
    MOVE G6A,113,  78, 145,  93,  90,  60
    MOVE G6C, 90,  40,  80,    ,    ,
    MOVE G6B,100,  40,  80,    ,    ,
    WAIT

    SPEED 7

    MOVE G6A, 113,  76, 142, 105,  90,  60
    MOVE G6D,  90,  96, 136,  85, 114,  60
    MOVE G6B, 100,  40,  80,    ,    ,
    MOVE G6C, 100,  40,  80,    ,    ,
    WAIT

    MOVE G6A, 100,  62, 146, 108, 100, 100
    MOVE G6D, 100,  88, 140,  86, 100, 100
    MOVE G6B,  90,  40,  80,    ,    ,
    MOVE G6C, 100,  40,  80,    ,    ,
    WAIT

    MOVE G6A,  90,  56, 143, 122, 114,  60
    MOVE G6D, 113,  80, 145,  90,  90,  60
    MOVE G6B,  80,  40,  80,    ,    ,
    MOVE G6C, 105,  40,  80,    ,    ,
    WAIT

    SPEED 13
    GOSUB backward_walk2

    SPEED 5
    GOSUB backward_walk1

    RETURN

backward_walk1:
    MOVE G6A,  85,  71, 152,  91, 112,  60
    MOVE G6D, 112,  76, 145,  93,  92,  60
    MOVE G6B, 100,  40,  80,    ,    ,
    MOVE G6C, 100,  40,  80,    ,    ,
    WAIT
    RETURN

backward_walk2:
    MOVE G6A,  90, 107, 105, 105, 114,  60
    MOVE G6D, 113,  78, 145,  93,  90,  60
    MOVE G6B,  90,  40,  80,    ,    ,
    MOVE G6C, 100,  40,  80,    ,    ,
    WAIT
    RETURN

    '================================================

forward_standup:

    SPEED 10

    MOVE G6A, 100, 130, 120,  80, 110, 100
    MOVE G6D, 100, 130, 120,  80, 110, 100
    MOVE G6B, 150, 160,  10,    ,    ,
    MOVE G6C, 150, 160,  10,    ,    ,
    WAIT

    MOVE G6A,  80, 155,  85, 150, 150, 100
    MOVE G6D,  80, 155,  85, 150, 150, 100
    MOVE G6B, 185,  40, 60,     ,    ,
    MOVE G6C, 185,  40, 60,     ,    ,
    WAIT

    MOVE G6A,  75, 165,  55, 165, 155, 100
    MOVE G6D,  75, 165,  55, 165, 155, 100
    MOVE G6B, 185,  10, 100,    ,    ,
    MOVE G6C, 185,  10, 100,    ,    ,
    WAIT

    MOVE G6A,  60, 165,  30, 165, 155, 100
    MOVE G6D,  60, 165,  30, 165, 155, 100
    MOVE G6B, 170,  10, 100,    ,    ,
    MOVE G6C, 170,  10, 100,    ,    ,
    WAIT

    MOVE G6A,  60, 165,  25, 160, 145, 100
    MOVE G6D,  60, 165,  25, 160, 145, 100
    MOVE G6B, 150,  60,  90,    ,    ,
    MOVE G6C, 150,  60,  90,    ,    ,
    WAIT

    MOVE G6A, 100, 155,  25, 140, 100, 100
    MOVE G6D, 100, 155,  25, 140, 100, 100
    MOVE G6B, 130,  50,  85,    ,    ,
    MOVE G6C, 130,  50,  85,    ,    ,
    WAIT

    RETURN

    '================================================
backward_standup:

    SPEED 10

    MOVE G6A, 100,  10, 100, 115, 100, 100
    MOVE G6D, 100,  10, 100, 115, 100, 100
    MOVE G6B, 100, 130,  10,    ,    ,
    MOVE G6C, 100, 130,  10,    ,    ,
    WAIT

    MOVE G6A, 100,  10,  83, 140, 100, 100
    MOVE G6D, 100,  10,  83, 140, 100, 100
    MOVE G6B,  20, 130,  10,    ,    ,
    MOVE G6C,  20, 130,  10,    ,    ,
    WAIT

    MOVE G6A, 100, 126,  60,  50, 100, 100
    MOVE G6D, 100, 126,  60,  50, 100, 100
    MOVE G6B,  20,  30,  90,    ,    ,
    MOVE G6C,  20,  30,  90,    ,    ,
    WAIT

    MOVE G6A, 100, 165,  70,  15, 100, 100
    MOVE G6D, 100, 165,  70,  15, 100, 100
    MOVE G6B,  30,  20,  95,    ,    ,
    MOVE G6C,  30,  20,  95,    ,    ,
    WAIT

    MOVE G6A, 100, 165,  40, 100, 100, 100
    MOVE G6D, 100, 165,  40, 100, 100, 100
    MOVE G6B, 110,  70,  50,    ,    ,
    MOVE G6C, 110,  70,  50,    ,    ,
    WAIT

    RETURN