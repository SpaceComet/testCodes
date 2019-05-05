Simple asm code that execute /bin/sh with linux syscall (32bits)
compile and link:
nasm -felf32 sys_execve.asm ; ld -m elf_i386 sys_execve.o -o sys_execve


Make a shellcode:
for i in `objdump -d sys_execve | tr '\t' ' ' | tr ' ' '\n' | egrep '^[0-9a-f]{2}$' ` ; do echo -n "\x$i" ; done
