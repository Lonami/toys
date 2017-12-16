.intel_syntax noprefix

.data
	.set N, 2000  #; number of programs
	.set M, 8     #; number of maximum connections
	conns: .space N*M*8, -1
	visit: .zero N  #; 1 if visitted, 0 otherwise

	fmt: .string "%d\n"
	readleft: .string "%ld <-> "
	readright: .string "%ld"
	tempn: .quad

.text
	.global main

#; rax has the number of the program to visit
#; rbx points to the base of conns
#; rdi points to the base of visit
visitall:
	push rsi
	push rax
	mov byte ptr [rdi+rax], 1  #; we have now visitted node rax
	mov rsi, rax
	imul rsi, M*8  #; rsi points to the row of conns we need
visitloop:
	add rsi, 8
	mov rax, [rbx+rsi]
	cmp rax, -1  #; if we reach a -1 we don't have more to test
	je visitdone
	test byte ptr [rdi+rax], 1
	jnz visitloop  #; we have already visitted this one
	call visitall  #; visit recursively
	jmp visitloop

visitdone:
	pop rax
	pop rsi
	ret

main:
	push rbx
	push r12
	push r13
	push r14

	lea rbx, conns[rip]  #; row position to memory
	xor r13, r13  #; row counter
readloop:
	lea rdi, readleft[rip]
	lea rsi, tempn[rip]
	xor rax, rax
	call scanf@PLT
	cmp al, -1
	je readdone
	mov rax, tempn[rip]

	mov [rbx+r13], rax
	mov r12, r13  #; column index
	add r13, M*8
	nop
readrightloop:
	lea rdi, readright[rip]
	lea rsi, tempn[rip]
	xor rax, rax
	call scanf@PLT

	add r12, 8
	mov rax, tempn[rip]
	mov [rbx+r12], rax

	call getchar@PLT
	cmp al, '\n'
	je readloop
	call getchar@PLT  #; space after ','
	jmp readrightloop

readdone:
	xor r8, r8  #; number of groups
	lea rbx, conns[rip]
	#; for the 2nd part we need to find which one isn't communicated
	#; yet and using that, visit all, then find the next, and so on.
	xor rax, rax
revisit:
	lea rdi, visit[rip]  #; scasb modifies rdi
	call visitall
	inc r8
	mov rcx, N
	xor al, al
	repne scasb  #; while we're finding <> 0, repeat

	test rcx, rcx
	jz show  #; actually, there may be another group left in (not the case :D)

	mov rax, N-1
	sub rax, rcx
	jmp revisit

show:
	lea rdi, fmt[rip]
	mov rsi, r8
	xor rax, rax
	#; need to figure out why this call isn't working...
	#; it's like if there's something left for scanf to read it won't work
	call printf@PLT

	pop r14
	pop r13
	pop r12
	pop rbx
	xor rax, rax
	ret
