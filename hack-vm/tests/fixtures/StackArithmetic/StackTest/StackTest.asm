// push constant 17
@17
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 17
@17
D = A
@SP
A = M
M = D
@SP
M = M + 1

// eq
@SP
AM = M - 1
D = M

@SP
A = M - 1
D = M - D
M = -1
@JEQ0
D; JEQ
@SP
A = M - 1
M = 0
(JEQ0)
// push constant 17
@17
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 16
@16
D = A
@SP
A = M
M = D
@SP
M = M + 1

// eq
@SP
AM = M - 1
D = M

@SP
A = M - 1
D = M - D
M = -1
@JEQ1
D; JEQ
@SP
A = M - 1
M = 0
(JEQ1)
// push constant 16
@16
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 17
@17
D = A
@SP
A = M
M = D
@SP
M = M + 1

// eq
@SP
AM = M - 1
D = M

@SP
A = M - 1
D = M - D
M = -1
@JEQ2
D; JEQ
@SP
A = M - 1
M = 0
(JEQ2)
// push constant 892
@892
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 891
@891
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
// push constant 891
@891
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 892
@892
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
@JLT1
D; JLT
@SP
A = M - 1
M = 0
(JLT1)
// push constant 891
@891
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 891
@891
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
@JLT2
D; JLT
@SP
A = M - 1
M = 0
(JLT2)
// push constant 32767
@32767
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 32766
@32766
D = A
@SP
A = M
M = D
@SP
M = M + 1

// gt
@SP
AM = M - 1
D = M

@SP
A = M - 1
D = M - D
M = -1
@JGT0
D; JGT
@SP
A = M - 1
M = 0
(JGT0)
// push constant 32766
@32766
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 32767
@32767
D = A
@SP
A = M
M = D
@SP
M = M + 1

// gt
@SP
AM = M - 1
D = M

@SP
A = M - 1
D = M - D
M = -1
@JGT1
D; JGT
@SP
A = M - 1
M = 0
(JGT1)
// push constant 32766
@32766
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 32766
@32766
D = A
@SP
A = M
M = D
@SP
M = M + 1

// gt
@SP
AM = M - 1
D = M

@SP
A = M - 1
D = M - D
M = -1
@JGT2
D; JGT
@SP
A = M - 1
M = 0
(JGT2)
// push constant 57
@57
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 31
@31
D = A
@SP
A = M
M = D
@SP
M = M + 1

// push constant 53
@53
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

// push constant 112
@112
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

// neg
@SP
A = M - 1
M = -M

// and
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M & D

// push constant 82
@82
D = A
@SP
A = M
M = D
@SP
M = M + 1

// or
@SP
AM = M - 1
D = M

@SP
A = M - 1
M = M | D

// not
@SP
A = M - 1
M = !M

