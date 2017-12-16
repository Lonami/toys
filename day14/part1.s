.intel_syntax noprefix

.data
    mydata: .string "AoC 2017"
    output: .space 16

.text
	.global main

main:
    lea rdi, mydata[rip]
    mov rsi, 8
    lea rdx, output[rip]
    call knothash

	xor rax, rax
	ret
