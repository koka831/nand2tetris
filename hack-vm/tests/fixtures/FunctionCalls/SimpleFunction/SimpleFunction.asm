// function SimpleFunction.test 2
(SimpleFunction.test)
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

// add
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M + D

// not
@SP
A = M - 1
M = !M

// push argument 0
@ARG
A = M

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

// push argument 1
@ARG
A = M
A = A + 1

D = M
@SP
A = M
M = D
@SP
M = M + 1

// sub
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M - D

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

