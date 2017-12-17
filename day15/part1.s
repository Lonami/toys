.intel_syntax noprefix

.data
    .set FACTORA, 16807
    .set FACTORB, 48271

    .set STARTA, 679
    .set STARTB, 771

    .set MODULO, 2147483647
    .set TIMES, 40000000

    fmt: .string "%d\n"

.text
	.global main

main:
    xor rsi, rsi  #; count
    mov r8, STARTA
    mov r9, STARTB
    mov r10, FACTORA
    mov r11, FACTORB
    mov rdi, MODULO
    mov rcx, TIMES
mainloop:
    xor rdx, rdx
    mov rax, r8
    mul r10
    xor rdx, rdx
    div rdi
    mov r8, rdx

    xor rdx, rdx
    mov rax, r9
    mul r11
    xor rdx, rdx
    div rdi
    mov r9, rdx

    cmp r8w, r9w
    jne nextiter
    inc rsi
nextiter:
    loop mainloop

    lea rdi, fmt[rip]
    #; rsi already has count
    xor rax, rax
    call printf@PLT

	xor rax, rax
	ret
