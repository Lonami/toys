.data
	fmt: .string "%d\n"
	vec: .zero 4096

.text
	.global main

main:
	push %rbp
	push %rbx

	lea vec(%rip), %rbp  # vec
	mov $0, %rbx  # item count

readloop:
	call getchar@PLT
	cmp $'0', %al
	jl endreadloop

	sub $'0', %al
	mov %al, (%rbp, %rbx)
	inc %rbx
	jmp readloop

endreadloop:
	mov %rbx, %r8   # r8 original size
	mov %rbx, %rcx  # rcx right index and counter
	shr %rbx        # rbx left index

	mov $0, %rsi  # result
	mov $0, %rax  # reset

countloop:
	mov -1(%rbp, %rcx), %al  # indices start at 0
	cmp -1(%rbp, %rbx), %al  # and we start at count
	jne countnext
	add %rax, %rsi

countnext:
	dec %rbx

	test %rbx, %rbx
	jnz countnext2
	mov %r8, %rbx  # wrap around the end

countnext2:
	loop countloop

	leaq fmt(%rip), %rdi
	mov $0, %eax
	call printf@PLT

	pop %rbx
	pop %rbp
	mov $0, %rax
	ret

