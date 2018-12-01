.intel_syntax noprefix

.data
	fmt: .string "%ld\n"
.text
	.global main


main:
    push rbx
    #; a -> rax
    #; b -> rbx
    #; c -> rcx
    #; d -> rdx
    #; e -> r8
    #; f -> r9
    #; g -> r10
    #; h -> r11
line1:
    mov rbx, 67
line2:
    mov rcx, rbx
line3:
    test rax, 0xffffffffffffffff
    jnz line5
line4:
    jmp line9
line5:
    imul rbx, 100
line6:
    sub rbx, -100000
line7:
    mov rcx, rbx
line8:
    sub rcx, -17000
line9:
    mov r9, 1
line10:
    mov rdx, 2
line11:
    mov r8, 2
line12:
    mov r10, rdx
line13:
    imul r10, r8
line14:
    sub r10, rbx
line15:
    test r10, 0xffffffffffffffff
    jnz line17
line16:
    mov r9, 0
line17:
    sub r8, -1
line18:
    mov r10, r8
line19:
    sub r10, rbx
line20:
    test r10, 0xffffffffffffffff
    jnz line12
line21:
    sub rdx, -1
line22:
    mov r10, rdx
line23:
    sub r10, rbx
line24:
    test r10, 0xffffffffffffffff
    jnz line11
line25:
    test r9, 0xffffffffffffffff
    jnz line27
line26:
    sub r11, -1
line27:
    mov r10, rbx
line28:
    sub r10, rcx
line29:
    test r10, 0xffffffffffffffff
    jnz line31
line30:
    jmp line33
line31:
    sub rbx, -17
line32:
    jmp line9
line33:
	lea rdi, fmt[rip]
	mov rsi, rax
	xor rax, rax
	call printf@PLT

	pop rbx
	xor rax, rax
	ret
