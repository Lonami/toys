.intel_syntax noprefix

.data
	fmt: .string "%d\n"
	vec: .zero 128  #; 16*8, 11 integers was enough actually

.text
	.global main

main:
	push rbp  #; base
	push rbx  #; index
	push r12  #; holds hash
	push r13  #; holds ok passphrases count

	lea rbp, vec[rip]
	mov r13, 0
startloop:
	mov rbx, 0

newwordloop:
	call getchar@PLT
	cmp al, '\n'  #; first char a newline? we're done
	je done

	#; we save a "hash", not the strings
	#; it's easier to compare hashes, held in r8
	imul rax, rax  #; square so 'ad' <> 'bc'
	mov r12, rax
mainloop:
	call getchar@PLT
	cmp al, '\n'
	je endloop
	cmp al, ' '
	je savehash

	imul rax, rax
	add r12, rax
	jmp mainloop

savehash:
	mov [rbp+rbx*8], r12
	inc rbx
	jmp newwordloop

endloop:
	mov [rbp+rbx*8], r12
	inc rbx

	#; reached newline while reading word
	#; so we need to check if passphrase is ok
	mov rsi, 0  #; i
leftcheck:
	mov rax, [rbp+rsi*8]  #; holds number to compare

	mov rdi, rsi
	inc rdi  #; j

	cmp rdi, rbx
	je checkgood
	rightcheck:
		cmp rax, [rbp+rdi*8]
		je failcheck
		inc rdi
		cmp rdi, rbx
		jl rightcheck
	inc rsi
	jmp leftcheck

checkgood:
	inc r13  #; check success, valid++

failcheck:
	jmp startloop

done:
	mov rsi, r13
	lea rdi, fmt[rip]
	mov rax, 0
	call printf@PLT

	pop r13
	pop r12
	pop rbx
	pop rbp
	ret

