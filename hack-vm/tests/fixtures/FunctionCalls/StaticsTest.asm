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
@2
D = D - A
@5
D = D - A
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
@2
D = D - A
@5
D = D - A
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
@Class1.get
1; JNE
(return-address3)
// call Class2.get 0
@return-address4
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
@Class2.get
1; JNE
(return-address4)
// label WHILE
(WHILE)
// goto WHILE
@WHILE
1; JNE
// function Class1.set 0
(Class1.set)

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

// function Class1.get 0
(Class1.get)

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

// function Class2.set 0
(Class2.set)

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

// function Class2.get 0
(Class2.get)

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

