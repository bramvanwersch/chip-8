## Instruction set for my version of a language interpreter
The instructions are formatted as follows; Each instruction has a name in capitals, directly after that is the opcode for the instruction and between brackets the different arguments in the order that they need to be written

- EXT(0000): exit the program
- STIS(Fx29) <x>: Set I = location of sprite for digit Vx. The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. 
- DRW(Dxyd) <x> <y> <d>: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision. The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen.
- STR(6xkk) <x> <kk>: Set Vx = kk. The interpreter puts the value kk into register Vx.
- 
