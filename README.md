# Cpu

This is a toy cpu that im working on, may create a version of it in logic world or something eventually

## Registers

Registers are 64 bits in size

| VALUE | NAME                | ASSEMBLY NAME | Extra info                                                                                                              |
| ----- | ------------------- | ------------- | ----------------------------------------------------------------------------------------------------------------------- |
| 0000  | ZERO                | rz            | Writing to it does nothing, reading from it gives the value 0                                                           |
| 0001  | INSTRUCTION POINTER | ri            | Stores the address of the next instruction                                                                              |
| 0010  | RETURN ADDRESS      | ra            | Set by the CALL instruction to the address after the CALL instruction, `COPY ra ri` can be used to return to the caller |
| 0011  | STACK POINTER       | rs            | For the address of the stack, its in the name                                                                           |
| 0100  | FRAME POINTER       | rf            | For keeping track of the base of the current stack frame for easy returning                                             |
| 0101  | REGISTER 1          | r1            | General Purpose Register                                                                                                |
| 0110  | REGISTER 2          | r2            | General Purpose Register                                                                                                |
| 0111  | REGISTER 3          | r3            | General Purpose Register                                                                                                |
| 1000  | REGISTER 4          | r4            | General Purpose Register                                                                                                |
| 1001  | REGISTER 5          | r5            | General Purpose Register                                                                                                |
| 1010  | REGISTER 6          | r6            | General Purpose Register                                                                                                |
| 1011  | REGISTER 7          | r7            | General Purpose Register                                                                                                |
| 1100  | REGISTER 8          | r8            | General Purpose Register                                                                                                |

## Instructions

|     | 0       | 1      | 2     | 3    | 4   | 5   | 6   | 7   | 8   | 9   | A   | B   | C   | D   | E   | F   |     |
| --- | ------- | ------ | ----- | ---- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0   | INVALID | ADD    | COPY  | HALT |     |     |     |     |     |     |     |     |     |     |     |     | 0   |
| 1   |         | SUB    | LOAD  | CALL |     |     |     |     |     |     |     |     |     |     |     |     | 1   |
| 2   |         | MUL    | READ  |      |     |     |     |     |     |     |     |     |     |     |     |     | 2   |
| 3   |         | DIVMOD | WRITE |      |     |     |     |     |     |     |     |     |     |     |     |     | 3   |
| 4   |         | AND    | PUSH  |      |     |     |     |     |     |     |     |     |     |     |     |     | 4   |
| 5   |         | OR     | POP   |      |     |     |     |     |     |     |     |     |     |     |     |     | 5   |
| 6   |         | XOR    |       |      |     |     |     |     |     |     |     |     |     |     |     |     | 6   |
| 7   |         | NOT    |       |      |     |     |     |     |     |     |     |     |     |     |     |     | 7   |
| 8   |         |        |       |      |     |     |     |     |     |     |     |     |     |     |     |     | 8   |
| 9   |         |        |       |      |     |     |     |     |     |     |     |     |     |     |     |     | 9   |
| A   |         |        |       |      |     |     |     |     |     |     |     |     |     |     |     |     | A   |
| B   |         |        |       |      |     |     |     |     |     |     |     |     |     |     |     |     | B   |
| C   |         |        |       |      |     |     |     |     |     |     |     |     |     |     |     |     | C   |
| D   |         |        |       |      |     |     |     |     |     |     |     |     |     |     |     |     | D   |
| E   |         |        |       |      |     |     |     |     |     |     |     |     |     |     |     |     | E   |
| F   |         |        |       |      |     |     |     |     |     |     |     |     |     |     |     |     | F   |
|     | 0       | 1      | 2     | 3    | 4   | 5   | 6   | 7   | 8   | 9   | A   | B   | C   | D   | E   | F   |     |

### Invalid

`INVALID`

This is an invalid instruction

### Add

`ADD a b o`

Adds registers `a` and `b` then puts the result in register `o`

### Sub

`SUB a b o`

Subtracts register `a` from register `b` then puts the result in register `o`

### Mul

`MUL a b o`

Multiples register `a` with register `b` then puts the result in register `o`

### DivMod

`DIVMOD a b o r`

Divides register `a` by register `b` then puts the result in register `o`, and the remainder in register `r`

### And

`AND a b o`

Preforms bitwise AND on registers `a` and `b` then puts the result in register `o`

### Or

`OR a b o`

Preforms bitwise OR on registers `a` and `b` then puts the result in register `o`

### Xor

`XOR a b o`

Preforms bitwise XOR on registers `a` and `b` then puts the result in register `o`

### Not

`NOT a o`

Preforms bitwise NOT on register `a` then puts the result in register `o`

### Copy

`COPY a b`

Copies register `a` to register `b`

### Load

`LOAD imm a`

Loads the immediate value `imm` into register `a`

### Read

`READ a b`

Treats register `a` as an address, then reads the value at that address and puts it in register `b`

### Write

`WRITE a b`

Writes the value in register `a` to the address stored in register `b`

### Push

`PUSH a`

Saves register `a` to a temporary, subtracts `sp` by 8, then writes the saved value of register `a` to the current `sp` value

### Pop

`POP a`

Reads the value pointed to by `sp` into register `a`, and then adds 8 to `sp`

### Halt

`HALT`

Halts the cpu

### Call

`CALL a`

Stores the current value of `ri` in `ra`, then jumps to the address stored in register `a`
