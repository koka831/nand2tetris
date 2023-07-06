@7
D=A
// push
@SP
A=M
M=D
@SP
M=M+1

@8
D=A
// push
@SP
A=M
M=D
@SP
M=M+1

// pop
@SP
A=M-1
D=M

@SP
A = M - 1
M = D + M

