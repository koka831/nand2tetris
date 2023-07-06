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

// push constant 6
@6
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 8
@8
D = A
@SP
A = M
M = D
@SP
M = M + 1

// call Class1.set 2
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

@7
D = A
@SP
D = M - D
@ARG
M = D

@SP
D = M
@LCL
M = D
@Class1.set
1; JNE
(return-address1)
// pop temp 0
@SP
AM = M - 1
D = M

@R5
M = D
// push constant 23
@23
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 15
@15
D = A
@SP
A = M
M = D
@SP
M = M + 1

// call Class2.set 2
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

@7
D = A
@SP
D = M - D
@ARG
M = D

@SP
D = M
@LCL
M = D
@Class2.set
1; JNE
(return-address2)
// pop temp 0
@SP
AM = M - 1
D = M

@R5
M = D
// call Class1.get 0
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
@Class1.get
1; JNE
(return-address3)
// call Class2.get 0
@return-address4
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
@Class2.get
1; JNE
(return-address4)
// label WHILE
(Sys.init$WHILE)
// goto WHILE
@Sys.init$WHILE
1; JNE
// function Class1.set 0
(Class1.set)
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

// pop static 0
@SP
AM = M - 1
D = M

@Class1.0
M = D
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

// pop static 1
@SP
AM = M - 1
D = M

@Class1.1
M = D
// push constant 0
@0
D = A
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

// function Class1.get 0
(Class1.get)
D = 0

// push static 0
@Class1.0
D = M
@SP
A = M
M = D
@SP
M = M + 1

// push static 1
@Class1.1
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

// function Class2.set 0
(Class2.set)
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

// pop static 0
@SP
AM = M - 1
D = M

@Class2.0
M = D
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

// pop static 1
@SP
AM = M - 1
D = M

@Class2.1
M = D
// push constant 0
@0
D = A
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

// function Class2.get 0
(Class2.get)
D = 0

// push static 0
@Class2.0
D = M
@SP
A = M
M = D
@SP
M = M + 1

// push static 1
@Class2.1
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

