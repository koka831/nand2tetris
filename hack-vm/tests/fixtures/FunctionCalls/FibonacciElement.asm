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
@Main.fibonacci
1; JNE
(return-address1)
// label WHILE
(WHILE)
// goto WHILE              
@WHILE
1; JNE
// function Main.fibonacci 0
(Main.fibonacci)

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

@IF_TRUE
D; JNE

// goto IF_FALSE
@IF_FALSE
1; JNE
// label IF_TRUE          
(IF_TRUE)
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

// label IF_FALSE         
(IF_FALSE)
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
@Main.fibonacci
1; JNE
(return-address0)
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
@Main.fibonacci
1; JNE
(return-address1)
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

