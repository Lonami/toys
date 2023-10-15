.intel_syntax noprefix

.data
	.set GROUP_BEGIN, '{'
	.set GROUP_END, '}'
	.set GARBAGE_BEGIN, '<'
	.set GARBAGE_END, '>'
	.set ESCAPE, '!'
	.set FINISH, '\n'

	fmt: .string "%d\n"

.text
	.global main

main:
	push r12
	push r13
	push r15

	xor r12, r12  #; bit flag status, 0th bit set means garbage
	xor r13, r13  #; how many groups are we inside of?
	xor r15, r15  #; score

mainloop:
	call getchar@PLT
	cmp al, FINISH
	je done
	cmp al, ESCAPE
	je skipnext  #; escaping should affect only inside garbage..?
	cmp al, GARBAGE_BEGIN
	je garbagebegin
	cmp al, GARBAGE_END
	je garbageend

	test r12b, 0x1
	jnz mainloop  #; inside garbage ignore groups

	cmp al, GROUP_BEGIN
	je groupbegin
	cmp al, GROUP_END
	je groupend

	jmp mainloop  #; other characters are ignored

garbagebegin:
	or r12b, 0x1
	jmp mainloop

garbageend:
	and r12b, 0xfe
	jmp mainloop

groupbegin:
	inc r13
	jmp mainloop

groupend:
	add r15, r13
	dec r13
	jmp mainloop

skipnext:
	call getchar@PLT
	jmp mainloop

done:
	lea rdi, fmt[rip]
	mov rsi, r15
	xor rax, rax
	call printf@PLT

	pop r15
	pop r13
	pop r12
	xor rax, rax
	ret
