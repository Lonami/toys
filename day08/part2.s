.intel_syntax noprefix

.data
	#; names are lowercase (fits in 5 bits) and up to 3 letters,
	#; so we have enough with just under two bytes (15 bits),
	#; which we can use as a way to access an array of 2^15 = 32768
	.set N, 32768
	variables: .zero N*8

	fmt: .string "%d\n"
	readn: .string "%ld"
	tempn: .quad 0
	best: .quad 0

.text
	.global main


#; reads a name and puts it in rax
readname:
	push rbx
	xor rbx, rbx
readnameloop:
	call getchar@PLT
	cmp al, ' '
	je readnamedone
	cmp al, '\n'
	je readnamedone

	sub al, '`'  #; '`' comes before 'a'
	shl rbx, 5
	or bl, al
	jmp readnameloop
readnamedone:
	mov rax, rbx
	pop rbx
	ret


main:
	push r12
	push r13
	push r14
	push r15

	xor r12, r12
mainloop:
	#; name
	call readname
	test rax, rax
	jz done
	mov r14, rax  #; hold the name on which operate in r14

	#; operation
	call getchar@PLT
	cmp al, 'i'  #; from inc (otherwise 'dec')
	sete r12b  #; r12 holds 1 if increase, 0 if decrease
	call getchar@PLT  #; 'n' or 'e'
	call getchar@PLT  #; 'c'

	#; amount
	lea rdi, readn[rip]
	lea rsi, tempn[rip]
	xor rax, rax
	call scanf@PLT
	mov r13, tempn[rip]  #; hold the amount in r13

	#; ' if '
	call getchar@PLT
	call getchar@PLT
	call getchar@PLT
	call getchar@PLT

	#; compare name
	call readname
	mov r15, rax  #; hold the name on which to compare in r15

	call getchar@PLT
	mov dh, al
	push rdx
	call getchar@PLT
	pop rdx
	mov dl, al  #; hold the operator in dx
	push rdx  #; getchar/scanf will use rdx but we need it

	cmp al, ' '
	je spaceread
	call getchar@PLT

spaceread:
	#; value to compare to
	lea rdi, readn[rip]
	lea rsi, tempn[rip]
	xor rax, rax
	call scanf@PLT
	call getchar@PLT  #; next line
	mov rax, tempn[rip]  #; hold the value to compare to in rax

	#; now we've read a single line (wew), prepare [rsi] -> variables
	pop rdx
	lea rsi, variables[rip]
	#; r14 name to modify
	#; r12 is 1 if increase, 0 if decrease
	#; r13 is the amount to increase/decrease r14
	#; r15 the name to compare
	#; dx is the operator
	#; rax the value to compare r15's value to
	cmp dh, '='
	je opif_equal
	cmp dh, '!'
	je opif_notequal
	cmp dh, '<'
	je opif_lessthan
	cmp dh, '>'
	je opif_greaterthan

	jmp done

opif_equal:
	cmp [rsi+r15*8], rax
	je opapply
	jmp mainloop

opif_notequal:
	cmp [rsi+r15*8], rax
	jne opapply
	jmp mainloop

opif_lessthan:
	cmp dl, '='
	je opif_lessequalthan
	cmp [rsi+r15*8], rax
	jl opapply
	jmp mainloop

opif_lessequalthan:
	cmp [rsi+r15*8], rax
	jle opapply
	jmp mainloop

opif_greaterthan:
	cmp dl, '='
	je opif_greaterequalthan
	cmp [rsi+r15*8], rax
	jg opapply
	jmp mainloop

opif_greaterequalthan:
	cmp [rsi+r15*8], rax
	jge opapply
	jmp mainloop

opapply:
	mov rax, [rsi+r14*8]
	test r12, r12
	jnz opapply_noneg
	neg r13
opapply_noneg:
	add rax, r13
	cmp rax, best[rip]
	jle opapply_done
	mov best[rip], rax
opapply_done:
	mov [rsi+r14*8], rax
	jmp mainloop

done:
	mov rsi, best[rip]
	lea rdi, fmt[rip]
	mov rax, 0
	call printf@PLT

	pop r15
	pop r14
	pop r13
	pop r12
	xor rax, rax
	ret
