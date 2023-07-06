// inject bootstrap
@256
D = A
@SP
M = D

@return-address0
D = A
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

@5
D = A
@SP
D = M - D
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
D = 0

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
D = A
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

@5
D = A
@SP
D = M - D
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
(Sys.init$LOOP)
// goto LOOP
@Sys.init$LOOP
1; JNE
// function Sys.main 5
(Sys.main)
D = 0
@SP
A = M
M = D
@SP
M = M + 1
@SP
A = M
M = D
@SP
M = M + 1
@SP
A = M
M = D
@SP
M = M + 1
@SP
A = M
M = D
@SP
M = M + 1
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
D = A
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

@6
D = A
@SP
D = M - D
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
// FRAME(= @R13) = LCL
@LCL
D = M
@R13
M = D

// FRAME - 5
@5
A = D - A
D = M
// RET = *(FRAME - 5)
@R14
M = D

@SP
A = M - 1
D = M
@R15
M = D

@ARG
D = M + 1
@SP
M = D

// SP - 1 = RET
@R15
D = M
@SP
A = M - 1
M = D

// THAT = *(FRAME - 1)
@R13
A = M - 1
D = M
@THAT
M = D

// THIS = *(FRAME - 2)
@2
D = A
@R13
A = M - D
D = M
@THIS
M = D

// ARG = *(FRAME - 3)
@3
D = A
@R13
A = M - D
D = M
@ARG
M = D

// LCL = *(FRAME - 4)
@4
D = A
@R13
A = M - D
D = M
@LCL
M = D

// goto RET
@R14
A = M
1;JNE

// function Sys.add12 0
(Sys.add12)
D = 0

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
// FRAME(= @R13) = LCL
@LCL
D = M
@R13
M = D

// FRAME - 5
@5
A = D - A
D = M
// RET = *(FRAME - 5)
@R14
M = D

@SP
A = M - 1
D = M
@R15
M = D

@ARG
D = M + 1
@SP
M = D

// SP - 1 = RET
@R15
D = M
@SP
A = M - 1
M = D

// THAT = *(FRAME - 1)
@R13
A = M - 1
D = M
@THAT
M = D

// THIS = *(FRAME - 2)
@2
D = A
@R13
A = M - D
D = M
@THIS
M = D

// ARG = *(FRAME - 3)
@3
D = A
@R13
A = M - D
D = M
@ARG
M = D

// LCL = *(FRAME - 4)
@4
D = A
@R13
A = M - D
D = M
@LCL
M = D

// goto RET
@R14
A = M
1;JNE

