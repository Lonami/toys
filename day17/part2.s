.intel_syntax noprefix

.data
    .set INPUT, 369
    .set SIZE, 50000000  #; size of the circular buffer array
    array: .zero SIZE*8

    fmt: .string "%d\n"

.text
	.global main

main:
    push rbx
    lea rbx, array[rip]  #; rbx -> base of [array]

    std  #; we're gonna be moving backwards when pushing the array items
    mov rax, 0  #; rax holds position
    mov r8, 1  #; r8 holds next value (& size)
    mov r9, SIZE-1  #; r9 holds total iterations to do
mainloop:
    #; insert current value at position + 1
    #; but FIRST (important) we need to push the other elements
    #; the amount of items to push is r8 (current size) - rax (i think)
    mov rcx, r8
    sub rcx, rax
    jz movedone
    lea rdi, array[rip]
    lea rdx, [r8*8]
    add rdi, rdx  #; rdi -> array[r8] which is current size which is end
    mov rsi, rdi
    sub rsi, 8  #; rsi -> array[r8]-8 which is previous position
    rep movsq  #; start copying [rdi--] = [rsi--]
movedone:
    #; new position is the POSITION at which we just inserted
    #; so we just increment one, insert there, and that's our pos
    inc rax
    mov [rbx+rax*8], r8
    inc r8  #; size (and next value) now changed

    add rax, INPUT  #; walk by INPUT
    xor rdx, rdx
    div r8
    mov rax, rdx    #; modulo current size

    dec r9
    jnz mainloop

done:
    cld
    mov rax, 0     #; look for this value
    mov rcx, SIZE  #; at most this many times
    lea rdi, array[rip]
    repne scasq

    mov rsi, [rdi]
    lea rdi, fmt[rip]
    xor rax, rax
    call printf@PLT

    pop rbx
	xor rax, rax
	ret
