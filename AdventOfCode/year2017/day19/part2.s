.intel_syntax noprefix

.data
	.set DIMENSIONS, 256
	maze: .zero DIMENSIONS*DIMENSIONS

	fmt: .string "%ld\n"

	.set EXIT, ' '
	.set VERTICAL, '|'
	.set HORIZONT, '-'
	.set TURN, '+'

	.set NORTH, 1
	.set EAST, 2
	.set SOUTH, 4
	.set WEST, 8

.text
	.global main

#; reads a maze into rdi
readmaze:
	push rbx
	push r12  #; row position
	push r13  #; maze position
	mov rbx, rdi
	xor r12, r12
	xor r13, r13
	rm_loop:
		call getchar@PLT
		cmp al, '\n'
		jle rm_nextrow
		mov [rbx+r13], al
		inc r13
		jmp rm_loop
	rm_nextrow:
		cmp al, '\n'
		jne rm_done
		add r12, DIMENSIONS
		mov r13, r12
		jmp rm_loop
rm_done:
	pop r13
	pop r12
	pop rbx
	ret


#; walks the maze starting at rdi
#; in rax returns steps walked
walkmaze:
	mov al, VERTICAL
	repne scasb
	dec rdi  #; it overshoots
	mov dl, SOUTH  #; dl will contain our direction
	xor rcx, rcx
wm_loop:
	test dl, NORTH
	jnz wm_walknorth
	test dl, EAST
	jnz wm_walkeast
	test dl, SOUTH
	jnz wm_walksouth
	#;test dl, WEST
	#;jnz wm_walkwest
wm_walkwest:
	dec rdi
	jmp wm_walkdone
wm_walknorth:
	sub rdi, DIMENSIONS
	jmp wm_walkdone
wm_walkeast:
	inc rdi
	jmp wm_walkdone
wm_walksouth:
	add rdi, DIMENSIONS
wm_walkdone:
	#; we have taken a step, now check the current character
	inc rcx
	mov al, [rdi]
	cmp al, EXIT
	je wm_done
	cmp al, VERTICAL
	je wm_loop
	cmp al, HORIZONT
	je wm_loop
	cmp al, TURN
	jne wm_loop  #; ignore letters
wm_turn:
	#; determine whether to turn left/right or up/down to a non-exit
	test dl, NORTH
	jnz wm_turnlr
	test dl, SOUTH
	jnz wm_turnlr
wm_turnud:
	mov dl, NORTH
	cmp byte ptr -DIMENSIONS[rdi], EXIT
	jne wm_loop
	mov dl, SOUTH  #; if it's not up then it's down
	jmp wm_loop
wm_turnlr:
	mov dl, WEST
	cmp byte ptr -1[rdi], EXIT
	jne wm_loop
	mov dl, EAST  #; if it's not left then it's right
	jmp wm_loop
wm_done:
	mov rax, rcx
	ret


main:
	lea rdi, maze[rip]
	call readmaze

	lea rdi, maze[rip]
	call walkmaze

	lea rdi, fmt[rip]
	mov rsi, rax
	xor rax, rax
	call printf@PLT

	xor rax, rax
	ret
