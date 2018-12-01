.data
	fmt: .string "%d\n"
	readn: .string "%d"
	tempn: .long 0

	.set N, 16  # matrix dimensions

.text
	.global main

main:
	push %rbp
	push %rbx
	push %r12  # min
	push %r15  # max
	push %r13  # result

	mov $0, %r13
	mov $N, %rbp  # row loop
rowloop:
	lea tempn(%rip), %rsi
	lea readn(%rip), %rdi
	mov $0, %rax
	call scanf@PLT
	mov tempn(%rip), %r12
	mov %r12, %r15

	mov $N-1, %rbx  # col loop
	colloop:
		lea tempn(%rip), %rsi
		lea readn(%rip), %rdi
		mov $0, %rax
		call scanf@PLT
		mov tempn(%rip), %rax
		cmp %r12, %rax
		jl ismin
		cmp %r15, %rax
		jg ismax
		jmp nextiter
	ismin:
		mov %rax, %r12
		jmp nextiter
	ismax:
		mov %rax, %r15
	nextiter:
		dec %rbx
		jnz colloop

	sub %r12, %r15  # max -= min
	add %r15, %r13  # sum += max

	dec %rbp
	jnz rowloop

	mov %r13, %rsi
	lea fmt(%rip), %rdi
	mov $0, %rax
	call printf@PLT

	pop %r13
	pop %r15
	pop %r12
	pop %rbx
	pop %rbp
	ret

