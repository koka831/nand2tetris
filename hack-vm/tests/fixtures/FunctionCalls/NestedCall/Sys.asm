@256
D = A
@SP
M = D
@return-address0
D = M
@SP
A = M
M = D
@SP
M = M + 1

@LCL
D = M
@SP
A = M
M = D
@SP
M = M + 1

@ARG
D = M
@SP
A = M
M = D
@SP
M = M + 1

@THIS
D = M
@SP
A = M
M = D
@SP
M = M + 1

@THAT
D = M
@SP
A = M
M = D
@SP
M = M + 1

@SP
D = A
@0
D = D - A
@5
D = D - A
@ARG
M = D

@SP
D = M
@LCL
M = D
@Sys.init
1; JNE
(return-address0)
// function Sys.init 0
(Sys.init)

// push constant 4000	
@4000
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop pointer 0
@SP
AM = M - 1
D = M

@THIS
M = D
// push constant 5000
@5000
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop pointer 1
@SP
AM = M - 1
D = M

@THAT
M = D
// call Sys.main 0
@return-address1
D = M
@SP
A = M
M = D
@SP
M = M + 1

@LCL
D = M
@SP
A = M
M = D
@SP
M = M + 1

@ARG
D = M
@SP
A = M
M = D
@SP
M = M + 1

@THIS
D = M
@SP
A = M
M = D
@SP
M = M + 1

@THAT
D = M
@SP
A = M
M = D
@SP
M = M + 1

@SP
D = A
@0
D = D - A
@5
D = D - A
@ARG
M = D

@SP
D = M
@LCL
M = D
@Sys.main
1; JNE
(return-address1)
// pop temp 1
@SP
AM = M - 1
D = M

@R6
M = D
// label LOOP
(LOOP)
// goto LOOP
@LOOP
1; JNE
// function Sys.main 5
(Sys.main)
D = 0
@SP
A = M
M = D
@SP
M = M + 1
D = 0
@SP
A = M
M = D
@SP
M = M + 1
D = 0
@SP
A = M
M = D
@SP
M = M + 1
D = 0
@SP
A = M
M = D
@SP
M = M + 1
D = 0
@SP
A = M
M = D
@SP
M = M + 1

// push constant 4001
@4001
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop pointer 0
@SP
AM = M - 1
D = M

@THIS
M = D
// push constant 5001
@5001
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop pointer 1
@SP
AM = M - 1
D = M

@THAT
M = D
// push constant 200
@200
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop local 1
@SP
AM = M - 1
D = M

@LCL
A = M
A = A + 1

M = D
// push constant 40
@40
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop local 2
@SP
AM = M - 1
D = M

@LCL
A = M
A = A + 1
A = A + 1

M = D
// push constant 6
@6
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop local 3
@SP
AM = M - 1
D = M

@LCL
A = M
A = A + 1
A = A + 1
A = A + 1

M = D
// push constant 123
@123
D = A
@SP
A = M
M = D
@SP
M = M + 1

// call Sys.add12 1
@return-address2
D = M
@SP
A = M
M = D
@SP
M = M + 1

@LCL
D = M
@SP
A = M
M = D
@SP
M = M + 1

@ARG
D = M
@SP
A = M
M = D
@SP
M = M + 1

@THIS
D = M
@SP
A = M
M = D
@SP
M = M + 1

@THAT
D = M
@SP
A = M
M = D
@SP
M = M + 1

@SP
D = A
@1
D = D - A
@5
D = D - A
@ARG
M = D

@SP
D = M
@LCL
M = D
@Sys.add12
1; JNE
(return-address2)
// pop temp 0
@SP
AM = M - 1
D = M

@R5
M = D
// push local 0
@LCL
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push local 1
@LCL
A = M
A = A + 1

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push local 2
@LCL
A = M
A = A + 1
A = A + 1

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push local 3
@LCL
A = M
A = A + 1
A = A + 1
A = A + 1

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push local 4
@LCL
A = M
A = A + 1
A = A + 1
A = A + 1
A = A + 1

D = M
@SP
A = M
M = D
@SP
M = M + 1

// add
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M + D

// add
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M + D

// add
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M + D

// add
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M + D

// return
// FRAME = LCL
@LCL
D = A
@FRAME
M = D

// FRAME - 5
D = A
@5
A = D - A // A = FRAME - 5
D = M
// RET = *(FRAME - 5)
@RET
M = D

// *ARG = pop()
@SP
AM = M - 1
D = M

@ARG
M = D

// SP = ARG + 1
D = A + 1
@SP
M = D

// THAT = *(FRAME - 1)
@FRAME
A = A - 1
D = M // D = *(FRAME - 1)
@THAT
M = D

// THIS = *(FRAME - 2)
@FRAME
A = A - 1
A = A - 1
D = M // D = *(FRAME - 2)
@THIS
M = D

// ARG = *(FRAME - 3)
@FRAME
A = A - 1
A = A - 1
A = A - 1
D = M // D = *(FRAME - 3)
@ARG

// LCL = *(FRAME - 4)
@FRAME
A = A - 1
A = A - 1
A = A - 1
A = A - 1
D = M // D = *(FRAME - 4)
@LCL

// goto RET
@RET
1; JNE

// function Sys.add12 0
(Sys.add12)

// push constant 4002
@4002
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop pointer 0
@SP
AM = M - 1
D = M

@THIS
M = D
// push constant 5002
@5002
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop pointer 1
@SP
AM = M - 1
D = M

@THAT
M = D
// push argument 0
@ARG
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push constant 12
@12
D = A
@SP
A = M
M = D
@SP
M = M + 1

// add
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M + D

// return
// FRAME = LCL
@LCL
D = A
@FRAME
M = D

// FRAME - 5
D = A
@5
A = D - A // A = FRAME - 5
D = M
// RET = *(FRAME - 5)
@RET
M = D

// *ARG = pop()
@SP
AM = M - 1
D = M

@ARG
M = D

// SP = ARG + 1
D = A + 1
@SP
M = D

// THAT = *(FRAME - 1)
@FRAME
A = A - 1
D = M // D = *(FRAME - 1)
@THAT
M = D

// THIS = *(FRAME - 2)
@FRAME
A = A - 1
A = A - 1
D = M // D = *(FRAME - 2)
@THIS
M = D

// ARG = *(FRAME - 3)
@FRAME
A = A - 1
A = A - 1
A = A - 1
D = M // D = *(FRAME - 3)
@ARG

// LCL = *(FRAME - 4)
@FRAME
A = A - 1
A = A - 1
A = A - 1
A = A - 1
D = M // D = *(FRAME - 4)
@LCL

// goto RET
@RET
1; JNE

