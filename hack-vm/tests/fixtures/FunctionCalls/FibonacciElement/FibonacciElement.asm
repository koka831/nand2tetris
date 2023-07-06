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

// push constant 4
@4
D = A
@SP
A = M
M = D
@SP
M = M + 1

// call Main.fibonacci 1
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
@Main.fibonacci
1; JNE
(return-address1)
// label WHILE
(Sys.init$WHILE)
// goto WHILE
@Sys.init$WHILE
1; JNE
// function Main.fibonacci 0
(Main.fibonacci)
D = 0

// push argument 0
@ARG
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push constant 2
@2
D = A
@SP
A = M
M = D
@SP
M = M + 1

// lt
@SP
AM = M - 1
D = M

@SP
A = M - 1
D = M - D
M = -1
@JLT0
D; JLT
@SP
A = M - 1
M = 0
(JLT0)
// if-goto IF_TRUE
@SP
AM = M - 1
D = M

@Main.fibonacci$IF_TRUE
D; JNE

// goto IF_FALSE
@Main.fibonacci$IF_FALSE
1; JNE
// label IF_TRUE
(Main.fibonacci$IF_TRUE)
// push argument 0
@ARG
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

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

// label IF_FALSE
(Main.fibonacci$IF_FALSE)
// push argument 0
@ARG
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push constant 2
@2
D = A
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

// call Main.fibonacci 1
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
@Main.fibonacci
1; JNE
(return-address2)
// push argument 0
@ARG
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push constant 1
@1
D = A
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

// call Main.fibonacci 1
@return-address3
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
@Main.fibonacci
1; JNE
(return-address3)
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

