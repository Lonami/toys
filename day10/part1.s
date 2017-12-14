.intel_syntax noprefix

.data
    .set N, 32  #; max number of lengths (though input is 16)
    values: .zero 256  #; input is 256 numbers, from 0..255
    lengths: .zero N  #; input is N numbers below 256

    fmt: .string "%d\n"
    readn: .string "%ld"
    tempn: .quad

.text
	.global main

main:
    push rbx
    push r12  #; number of lengths
    xor r12, r12

    #; initialize lengths
readloop:
    lea rdi, readn[rip]
    lea rsi, tempn[rip]
    xor rax, rax
    call scanf@PLT
    mov rax, tempn[rip]
    lea rsi, lengths[rip] 
    mov [rsi+r12], al
    inc r12
    call getchar@PLT
    cmp al, '\n'
    jne readloop

    lea rsi, values[rip]  #; rsi -> values
    lea rdi, lengths[rip]  #; rdi -> lengths

    #; initialize values
    mov rcx, 255
valuesloop:
    mov [rsi+rcx], cl
    loop valuesloop

    #; now we have everything so let's define other indices
    xor rbx, rbx  #; bh holds read length, bl how many swap iterations left
    xor rdx, rdx  #; access to rdi -> lengths
    xor rax, rax  #; al holds the left index while reversing
    xor rcx, rcx  #; cl holds the right index while reversing
    xor r8, r8    #; r8b holds the current index
    xor r9, r9    #; r9b holds skip value
    xor r10, r10  #; r10b holds values from memory when swapping

workloop:
    #; the great thing of a 256-sized list is we get modulo 256 for free
    mov al, r8b
    mov cl, r8b
    mov bh, [rdi+rdx]
    add cl, bh
    dec cl  #; indices start at 0
    mov bl, bh
    shr bl  #; we just need half the iterations to swap
    jz revloopnext  #; or even none if length <= 1

revloop:
    mov r10b, [rsi+rax]
    xchg [rsi+rcx], r10b
    mov [rsi+rax], r10b
    inc al
    dec cl
    dec bl
    jnz revloop

revloopnext:
    mov bl, bh
    add r8b, bl
    add r8b, r9b

    inc r9b  #; larger skip value
    inc rdx  #; next length
    dec r12  #; one item less
    jnz workloop

    lea rdi, fmt[rip]
    mov al, 0[rsi]
    mov cl, 1[rsi]
    mov rsi, rax
    imul rsi, rcx
    xor rax, rax
    call printf@PLT

    pop r12
    pop rbx
	xor rax, rax
	ret
