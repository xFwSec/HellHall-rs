.globl HellHall
.section .text
HellHall:
	movabs rax, JMPINSTRUCT
	mov r11, rax
	xor rax, rax
	mov r10, rcx
	movabs eax, SSNNUMBER
	jmp r11
