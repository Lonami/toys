.intel_syntax noprefix

.data
    .set MAXR2, 8
    .set MAXR3, 128

    #; 0 means off, 1 means on, -1 means not set
    rules2: .space MAXR2*13, -1  #; 2*2 + 3*3 = 4 +  9
    rules3: .space MAXR3*25, -1  #; 3*3 + 4*4 = 9 + 16

.text
	.global main


#; rdi -> [rules2] base
#; rsi -> [rules3] base
readrules:
    push rbx
    push r12
    push r13
    mov r12, rdi
    mov r13, rsi
rr_loop:
    call getchar@PLT
    cmp al, '\n'
    jle rr_done
    #; start by assuming it's a 3x3 rule
    cmp al, '#'
    sete 0[r13]
    call getchar@PLT
    cmp al, '#'
    sete 1[r13]
    call getchar@PLT
    cmp al, '/'
    je rr_rule2
rr_rule3:
    cmp al, '#'
    sete 2[r13]
    add r13, 3
rr_rule3loop:
    call getchar@PLT
    cmp al, '\n'
    je rr_loop
    cmp al, '.'
    je rr_rule3apply
    cmp al, '#'
    jne rr_rule3loop
rr_rule3apply:
    cmp al, '#'
    sete [r13]
    inc r13
    jmp rr_rule3loop

rr_rule2:
    #; need to copy from rule3 -> rule2
    mov al, 0[r13]
    mov 0[r12], al
    mov byte ptr 0[r13], -1
    mov al, 1[r13]
    mov 1[r12], al
    mov byte ptr 1[r13], -1
    add r12, 2
rr_rule2loop:
    call getchar@PLT
    cmp al, '\n'
    je rr_loop
    cmp al, '.'
    je rr_rule2apply
    cmp al, '#'
    jne rr_rule2loop
rr_rule2apply:
    cmp al, '#'
    sete [r12]
    inc r12
    jmp rr_rule2loop

rr_done:
    pop r13
    pop r12
    pop rbx
    ret


main:
    lea rdi, rules2[rip]
    lea rsi, rules3[rip]
    call readrules

	xor rax, rax
	ret
