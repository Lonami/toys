.intel_syntax noprefix

.data
	states: .byte 1, 1, 1, 0, 0, 2, -1, -1, 1, 0, 0, 1, 1, 3, -1, -1, 0, 0, 1, 0, 0, 4, -1, -1, 1, 1, 0, 0, 1, 1, -1, -1, 1, 0, 5, 1, 0, 2, -1, -1, 1, 1, 3, 1, 1, 0, -1, -1
	#;0 write val ^  ^  ^  ^  ^  ^   ^   ^
	#;0 left0/right1 |  |  |  |  |    \---\- padding
	#;0      next state |  |  |  |
	#;1          write val |  |  |
	#;1   left = 0, right = 1 |  |
	#;1               next state |
	#;^ if this value
	.set FIRST_STATE, 0
	.set ITERATIONS, 12481997

	.set N, 100000
	tape: .zero N

	fmt: .string "%d\n"

.text
	.global main

main:
	lea rsi, states[rip]    #; rsi = base to states
	lea rdi, tape+N/2[rip]  #; rdi = pointer to tapes
	mov rdx, FIRST_STATE    #; rdx = current state

	mov rcx, ITERATIONS     #; rcx = number of iterations
mainloop:
	#; select first 3 values or second 3 values, depending on tape
	#; r8b = value to write
	#; r9b = move left (0) or right (1)
	#; rdx = next state
	test byte ptr [rdi], 1
	jnz isone
iszero:
	mov r8b, 0[rsi+rdx*8]
	mov r9b, 1[rsi+rdx*8]
	movzx rdx, byte ptr 2[rsi+rdx*8]
	jmp nextiter
isone:
	mov r8b, 3[rsi+rdx*8]
	mov r9b, 4[rsi+rdx*8]
	movzx rdx, byte ptr 5[rsi+rdx*8]
nextiter:
	mov [rdi], r8b
	test r9b, 1
	jnz moveright
moveleft:
	dec rdi
	jmp realnextiter
moveright:
	inc rdi
realnextiter:
	loop mainloop

	lea rdi, tape[rip]  #; rdi = base to tapes
	xor rsi, rsi  #; count 1's in rsi
	mov rcx, N
countloop:
	test byte ptr -1[rdi+rcx], 1
	jz nextcount
	inc rsi
nextcount:
	loop countloop

	lea rdi, fmt[rip]
	#; rsi has count
	xor rax, rax
	call printf@PLT

	xor rax, rax
	ret
