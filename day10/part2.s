.intel_syntax noprefix

.data
    values: .zero 256  #; input is 256 numbers, from 0..255
    lengths: .zero 64  #; input is 64 ascii characters or less
    collapsed: .zero 16  #; collapsed bits

    fmt: .string "%02x"
    readn: .string "%ld"
    tempn: .quad

.text
	.global main

main:
    push rbx
    push r12
    push r13

    #; for now r12 will be the index to lengths
    lea r12, lengths[rip]
readloop:
    call getchar@PLT
    cmp al, '\n'
    je readdone
    mov [r12], al
    inc r12
    jmp readloop
readdone:
    #; always add `17, 31, 73, 47, 23' to the end
    mov byte ptr 0[r12], 17
    mov byte ptr 1[r12], 31
    mov byte ptr 2[r12], 73
    mov byte ptr 3[r12], 47
    mov byte ptr 4[r12], 23
    add r12, 5

    lea rsi, values[rip]  #; rsi -> values
    lea rdi, lengths[rip]  #; rdi -> lengths
    sub r12, rdi
    #; initialize values
    mov rcx, 255
valuesloop:
    mov [rsi+rcx], cl
    loop valuesloop

    #; now we have everything so let's define other indices
    xor rbx, rbx  #; bh holds read length, bl how many swap iterations left
    xor rax, rax  #; al holds the left index while reversing
    xor rcx, rcx  #; cl holds the right index while reversing
    xor r8, r8    #; r8b holds the current index, preserved accross loops
    xor r9, r9    #; r9b holds skip value, preserved accross loops
    xor r10, r10  #; r10b holds values from memory when swapping
    mov r13, 64   #; number of rounds
roundloop:
    xor rdx, rdx  #; access to rdi -> lengths
    mov r11, r12  #; r11 holds length count

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
        dec r11  #; one item less
        jnz workloop

    dec r13
    jnz roundloop

    #; now we collapse the 256 bytes into 16, on 'collapsed' for simplicity
    lea rsi, values[rip]
    lea rdi, collapsed[rip]
    mov ch, 16  #; 16 times
collapseraxloop:
    mov al, [rsi]
    inc rsi
    mov cl, 15  #; 15+1 (1st already in al) bytes
    collapseinrax:
        xor al, [rsi]
        inc rsi
        dec cl
        jnz collapseinrax

    mov [rdi], al
    inc rdi
    dec ch
    jnz collapseraxloop

    #; rbx holds the index to the hash
    #; r12 items left
    #; r13 for 8 -> 64 bits
    mov r12, 16
    xor r13, r13
    lea rbx, collapsed[rip]

showloop:
    lea rdi, fmt[rip]
    mov r13b, [rbx]
    mov rsi, r13
    xor rax, rax
    call printf@PLT
    inc rbx
    dec r12
    jnz showloop

    mov rdi, '\n'
    call putchar@PLT

    pop r13
    pop r12
    pop rbx
	xor rax, rax
	ret
