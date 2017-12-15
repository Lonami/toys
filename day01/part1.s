.data
	fmt: .string "%d\n"

.text
	.global main

main:
	push %rbx
	push %r15

	call getchar@PLT

	mov %al, %bh  # first
	mov %al, %bl  # last
	mov $0, %r15  # counter

loop:
	call getchar@PLT
	cmp $'0', %al
	jl endloop

	xchg %al, %bl
	cmp %al, %bl
	jne loop

	sub $'0', %rax
	add %rax, %r15

	jmp loop
endloop:
	cmp %bl, %bh
	jne show

	mov $0, %bh
	sub $'0', %bl
	add %rbx, %r15

show:
	mov %r15, %rsi
	leaq fmt(%rip), %rdi
	mov $0, %eax
	call printf@PLT

	pop %r15
	pop %rbx
	mov $0, %rax
	ret

