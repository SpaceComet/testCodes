section .text
  global _start

_start:
  xor eax, eax

  ; Push //bin/sh to the stack
  push eax
  push 0x68732f6e ;n/sh
  push 0x69622f2f ;//bi

  ; Get the pointer to ebx
  mov ebx, esp

  ; sys_execve
  mov al, 0x0b
  int 0x80
