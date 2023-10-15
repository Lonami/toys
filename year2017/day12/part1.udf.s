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
	xor rax, rax
	lea rbx, conns[rip]
	lea rdi, visit[rip]
	call visitall
	#; now we just count how many are visitted
	lea rdi, visit[rip]  #; access
	xor rsi, rsi  #; accumulator
	mov rcx, N  #; counter

countgroups:
	test byte ptr -1[rdi+rcx], 1
	jz countgroupsnext
	inc rsi
countgroupsnext:
	loop countgroups

	lea rdi, fmt[rip]
	#; rsi already has count
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
