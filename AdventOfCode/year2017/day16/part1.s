.intel_syntax noprefix

.data
	readsingle: .string "%ld"
	readpair: .string "%ld/%ld"
	left: .zero 8
	right: .zero 8

	.set N, 16
	programs: .ascii "abcdefghijklmnop"
	tmparray: .space N  #; spinning

.text
	.global main

main:
	push rbx
	push r12
	lea rbx, programs[rip]

mainloop:
	call getchar@PLT
	cmp al, 's'
	je spin
	cmp al, 'x'
	je exchange
	#;cmp al, 'p'
	#;je partner  #; assume well formed input

partner:
	call getchar@PLT
	#; find first partner memory position, save in r12
	lea rdi, programs[rip]
	mov rcx, N
	repne scasb
	mov r12, rdi

	call getchar@PLT  #; '/'
	call getchar@PLT
	lea rdi, programs[rip]
	mov rcx, N
	repne scasb

	#; xcgh -1[rdi], -1[r12], as it overshoots
	mov cl, -1[rdi]
	xchg -1[r12], cl
	mov -1[rdi], cl
	jmp nextiter

spin:
	lea rdi, readsingle[rip]
	lea rsi, left[rip]
	xor rax, rax
	call scanf@PLT
	mov rax, left[rip]

	lea rsi, programs+N[rip]
	sub rsi, rax
	lea rdi, tmparray[rip]
	mov rcx, rax
	rep movsb
	lea rsi, programs[rip]
	mov rcx, N
	sub rcx, rax
	rep movsb

	lea rsi, tmparray[rip]
	lea rdi, programs[rip]
	mov rcx, N
	rep movsb
	jmp nextiter

exchange:
	lea rdi, readpair[rip]
	lea rsi, left[rip]
	lea rdx, right[rip]
	xor rax, rax
	call scanf@PLT
	mov rax, left[rip]
	mov rdx, right[rip]
	mov cl, [rbx+rax]
	xchg [rbx+rdx], cl
	mov [rbx+rax], cl

nextiter:
	call getchar@PLT
	cmp al, ','
	je mainloop

done:
	mov r12, 0

showloop:
	mov rdi, [rbx+r12]
	call putchar@PLT
	inc r12
	cmp r12, N
	jne showloop
	mov rdi, '\n'
	call putchar@PLT

	pop r12
	pop rbx
	xor rax, rax
	ret
