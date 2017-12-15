.intel_syntax noprefix

.data
	fmt: .string "%lld\n"
	vec: .zero 2048*8
	tempn: .long 0

.text
	.global main

main:
	push rbp  #; base
	push rbx  #; index

	lea rbp, vec[rip]
	mov rbx, 0
readloop:
	lea rdi, fmt[rip]
	lea rsi, tempn[rip]
	mov rax, 0
	call scanf@PLT
	cmp rax, 0
	jle allread

	mov rax, tempn[rip]
	mov [rbp+rbx*8], rax
	inc rbx
	jmp readloop

allread:
	mov rcx, rbx  #; rcx = item count
	mov rsi, 0  #; steps taken
	mov rbx, 0

walk:
	mov rax, [rbp+rbx*8]
	inc QWORD PTR [rbp+rbx*8]
	add rbx, rax

	inc rsi
	cmp rbx, rcx
	jl walk

	lea rdi, fmt[rip]
	mov rax, 0
	call printf@PLT

	pop rbx
	pop rbp
	ret

