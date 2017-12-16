.intel_syntax noprefix

.data
	.set N, 2000  #; number of programs
	matrix: .zero N*N  #; just bytes to save 1 if connected 0 otherwise
	warshall: .zero N*N  #; warshall path matrix

	fmt: .string "%d\n"
	readleft: .string "%ld <-> "
	readright: .string "%ld"
	tempn: .quad

.text
	.global main

#; r12 has one value, r13 has the other
#; call this to save them in the matrix
savematrix:
	lea rcx, matrix[rip]
	mov rax, r12
	imul rax, N
	add rax, rcx
	mov byte ptr [rax+r13], 1  #; r12 row -> r13 col
	mov rax, r13
	imul rax, N
	add rax, rcx
	mov byte ptr [rax+r12], 1  #; r13 row -> r12 col
	ret

#; fills in the warshall matrix
calcwarshall:
	mov rcx, N*N
	lea rsi, matrix[rip]
	lea rdi, warshall[rip]
	rep movsb
	lea rsi, warshall[rip]  #; rsi -> warshall base
	#; for k in 0..N:
	#;   for i in 0..N:
	#;     for j in 0..N:
	#;        warshall[i][j] |= warshall[i][k] & warshall[k][j]
	#; setup loops
	mov r10, N     #; r10 -> k
cw_kloop:
	mov r8, N      #; r8  -> i
	cw_iloop:
		mov r9, N  #; r9  -> j
		cw_jloop:
			#; rax = row i memory address (-1 because we go from N..1)
			lea rax, -1[r8]
			imul rax, N
			add rax, rsi
			test byte ptr -1[rax+r9], 1
			jnz cw_nextj  #; no need to access warshall[i][k] & warshall[k][j]

			test byte ptr -1[rax+r10], 1
			jz cw_nextj  #; short-circuit, warshall[i][k] is zero, & fails

			#; rdx = row k memory address
			lea rdx, -1[r10]
			imul rdx, N
			add rdx, rsi
			test byte ptr -1[rdx+r9], 1
			jz cw_nextj  #; it's zero, so & fails as well

			#; if it didn't fail then we set the byte to 1
			mov byte ptr -1[rax+r9], 1
		cw_nextj:
			dec r9
			jnz cw_jloop
		dec r8
		jnz cw_iloop
	dec r10
	jnz cw_kloop
	ret

main:
	push rbx
	push r12
	push r13
	push r14

	lea rbx, matrix[rip]
readloop:
	lea rdi, readleft[rip]
	lea rsi, tempn[rip]
	xor rax, rax
	call scanf@PLT
	cmp al, -1
	je readdone
	mov r12, tempn[rip]

readrightloop:
	lea rdi, readright[rip]
	lea rsi, tempn[rip]
	xor rax, rax
	call scanf@PLT
	mov r13, tempn[rip]
	call savematrix
	call getchar@PLT
	cmp al, '\n'
	je readloop
	call getchar@PLT  #; space after ','
	jmp readrightloop

readdone:
	call calcwarshall
	#; now we just count how many are connected with 0
	lea rdi, warshall[rip]  #; access
	xor rsi, rsi  #; accumulator
	mov rcx, N  #; counter

countgroups:
	test byte ptr [rdi], 1
	jz countgroupsnext
	inc rsi
countgroupsnext:
	add rdi, N
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
