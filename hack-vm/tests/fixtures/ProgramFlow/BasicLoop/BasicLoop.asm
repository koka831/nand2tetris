// push constant 0
@0
D = A
@SP
A = M
M = D
@SP
M = M + 1

// pop local 0
@SP
AM = M - 1
D = M

@LCL
A = M

M = D
// label LOOP_START
(LOOP_START)
// push argument 0
@ARG
A = M

D = M
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

// add
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M + D

// pop local 0
@SP
AM = M - 1
D = M

@LCL
A = M

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
// push argument 0
@ARG
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

// if-goto LOOP_START
@SP
AM = M - 1
D = M

@LOOP_START
D; JNE

// push local 0
@LCL
A = M

D = M
@SP
A = M
M = D
@SP
M = M + 1

