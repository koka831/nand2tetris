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

// pop pointer 1
@SP
AM = M - 1
D = M

@THAT
M = D
// push constant 0
@0
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop that 0
@SP
AM = M - 1
D = M

@THAT
A = M

M = D
// push constant 1
@1
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop that 1
@SP
AM = M - 1
D = M

@THAT
A = M
A = A + 1

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

// pop argument 0
@SP
AM = M - 1
D = M

@ARG
A = M

M = D
// label MAIN_LOOP_START
(MAIN_LOOP_START)
// push argument 0
@ARG
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

// if-goto COMPUTE_ELEMENT
@SP
AM = M - 1
D = M

@COMPUTE_ELEMENT
D; JNE

// goto END_PROGRAM
@END_PROGRAM
1; JNE
// label COMPUTE_ELEMENT
(COMPUTE_ELEMENT)
// push that 0
@THAT
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

// push that 1
@THAT
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

// pop that 2
@SP
AM = M - 1
D = M

@THAT
A = M
A = A + 1
A = A + 1

M = D
// push pointer 1
@THAT
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

// add
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M + D

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

// pop argument 0
@SP
AM = M - 1
D = M

@ARG
A = M

M = D
// goto MAIN_LOOP_START
@MAIN_LOOP_START
1; JNE
// label END_PROGRAM
(END_PROGRAM)
