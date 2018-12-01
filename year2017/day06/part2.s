.intel_syntax noprefix

.data
	fmt: .string "%d\n"

	.set N, 16  #; amount of numbers to be read
	vec: .zero N*8  #; numbers read from stdin
	hashes: .zero 10240*8  #; max configs

	readn: .string "%ld"
	tempn: .long 0

.text
	.global main

#; rbp holds the vector of values
#;
#; uses rax (temp), rbx (iter index),
#; rcx (highest val), rdx (best index), r9 (zero)
balance:
	mov r9, 0
	#; first we find the largest value
	mov rcx, [rbp]
	mov rdx, 0

	mov rbx, 1
balancesearch:
	mov rax, [rbp+rbx*8]
	cmp rcx, rax
	jge balancesearchnext
	mov rcx, rax
	mov rdx, rbx

balancesearchnext:
	inc rbx
	cmp rbx, N
	jne balancesearch

	mov QWORD PTR [rbp+rdx*8], 0
	#; now we use best index to iterate, no
	#; need to move it to rbx and use that.
	inc rdx
balanceloop:
	cmp rdx, N
	cmove rdx, r9  #; wrap back to 0 on N
	inc QWORD PTR [rbp+rdx*8]
	inc rdx
	loop balanceloop
	ret

#; rbp holds the vector of values
#; rdi holds the vector of hashes
#; r15 contains the hash multiplier
#;
#; returns the hash in rax, uses rbx (index)
#; rdi will be increased by 8 (next index)
savehash:
	mov rax, [rbp]
	mov rbx, 1

savehashloop:
	imul rax, r15
	add rax, [rbp+rbx*8]
	inc rbx
	cmp rbx, N
	jne savehashloop

	mov [rdi], rax
	add rdi, 8
	ret

main:
	push rbp  #; base
	push rbx  #; index
	push r15  #; holds hash multiplier

	lea rbp, vec[rip]
	mov rbx, 0
	mov r15, 0
readloop:
	lea rdi, readn[rip]
	lea rsi, tempn[rip]
	mov rax, 0
	call scanf@PLT
	mov rax, tempn[rip]
	mov [rbp+rbx*8], rax
	add r15, rax

	inc rbx
	cmp rbx, N
	jne readloop

	#; increment hash multiplier one more to avoid collisions
	inc r15

	lea r8, hashes[rip]
	mov rdi, r8
	call savehash

	mov rsi, 0  #; balance count
mainloop:
	call balance
	inc rsi
	call savehash

	mov rcx, rsi  #; balance happens to be hashes-1
	dec rcx  #; indices start at zero though
	jz mainloop
checkhash:
	cmp rax, [r8+rcx*8]
	je hashfound
	loop checkhash
	jmp mainloop

hashfound:
	sub rsi, rcx
	lea rdi, fmt[rip]
	mov rax, 0
	call printf@PLT

	pop r15
	pop rbx
	pop rbp
	ret

