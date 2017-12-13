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
    xor r13, r13  #; current garbage score
	xor r15, r15  #; removed garbage

mainloop:
	call getchar@PLT
	cmp al, FINISH
	je done
	cmp al, ESCAPE
	je skipnext
    inc r13  #; every character scores (set to 0 if garbage begin anyway)
	cmp al, GARBAGE_BEGIN
	je garbagebegin
	cmp al, GARBAGE_END
	je garbageend

	jmp mainloop  #; other characters are ignored

garbagebegin:
    test r12b, 0x1
    jnz mainloop  #; bit already set, don't reset garbage counter
    xor r13, r13  #; set counter to zero if we weren't inside garbage
	or r12b, 0x1
	jmp mainloop

garbageend:
    dec r13  #; delimiters don't count
    add r15, r13
	and r12b, 0xfe
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
