.data
	fmt: .string "%d\n"
	readn: .string "%d"
	tempn: .long 0

	.set N, 16  # matrix dimensions
	rowvec: .space N*8

.text
	.global main

main:
	push %rbp
	push %rbx
	push %r12
	push %r13

	# setup, row vector stores items in reverse
	# but the order doesn't really matter anyway
	lea rowvec(%rip), %r12
	mov $0, %r13  # solution

	# enter read N rows loop with N columns
	mov $N, %rbp  # row loop
rowloop:
	mov $N, %rbx  # col loop
	colloop:
		lea tempn(%rip), %rsi
		lea readn(%rip), %rdi
		mov $0, %rax
		call scanf@PLT
		mov tempn(%rip), %rax
		mov %rax, (%r12, %rbx, 8)
	nextiter:
		dec %rbx
		jnz colloop


	# now we find the pair that's divisible inside rowloop
	mov $N, %rdi  # right most loop down to 1
rightloop:
	mov %rdi, %rsi  # left most loop down to 0
	dec %rsi
	leftloop:
		mov $0, %rdx
		mov -1(%r12, %rdi, 8), %rax
		mov -1(%r12, %rsi, 8), %rcx
		cmp %rcx, %rax
		jge noswap  # divident has to be smaller
		xchg %rcx, %rax
	noswap:
		div %rcx
		test %rdx, %rdx
		jz found

		dec %rsi
		jnz leftloop

	dec %rdi
	cmp $1, %rdi
	je nofound
	jmp rightloop

	found:
	add %rax, %r13

	nofound:  # should not happen but well
	# next row iteration
	dec %rbp
	jnz rowloop

	mov %r13, %rsi
	lea fmt(%rip), %rdi
	mov $0, %rax
	call printf@PLT

	pop %r13
	pop %r12
	pop %rbx
	pop %rbp
	ret

