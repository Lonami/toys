.intel_syntax noprefix

.data
	.set N, 100

	fmt: .string "%d\n"
	readfmt: .string "%d/%d "
	left: .space 8
	right: .space 8

	#; bytes are enough
	bridges: .zero 2*N
	used: .zero N

	best: .zero 8

.text
	.global main


readdata:
	push rbx
	lea rbx, bridges[rip]
rd_loop:
	lea rdi, readfmt[rip]
	lea rsi, left[rip]
	lea rdx, right[rip]
	xor rax, rax
	call scanf@PLT
	cmp eax, 0
	jle rd_done
	lea rsi, left[rip]
	mov al, [rsi]
	mov 0[rbx], al
	lea rsi, right[rip]
	mov al, [rsi]
	mov 1[rbx], al
	add rbx, 2
	jmp rd_loop
rd_done:
	mov byte ptr 0[rbx], -1
	pop rbx
	ret


#; rax <- score
getscore:
	lea rsi, bridges[rip]
	lea rdi, used[rip]
	xor rax, rax
gs_loop:
	test byte ptr [rdi], 0xff
	jz gs_noscore
	movzx rdx, byte ptr 0[rsi]
	add rax, rdx
	movzx rdx, byte ptr 1[rsi]
	add rax, rdx
gs_noscore:
	add rsi, 2
	add rdi, 1
	cmp byte ptr [rsi], -1
	jne gs_loop
	ret


#; dx -> bridge values that can match to
placebridge:
	push rbx
	push r12
	push r13
	push r14
	lea r12, bridges[rip]  #; r12 = bridges pointer
	lea r13, used[rip]  #; r13 = used pointer
	xor r14b, r14b  #; r14b = any valid
	mov bx, dx  #; bl, bh = left, right
pb_searchloop:
	test byte ptr [r13], 0xff
	jnz pb_nextiter  #; in use
	mov ax, [r12]  #; load two bytes at once
	mov dx, ax  #; on dx, set to "-1" the bits that were used
	mov dl, -1
	cmp al, bl
	je pb_validbridge
	cmp al, bh
	je pb_validbridge
	mov dl, al  #; dl isn't used, restore, try dh
	mov dh, -1
	cmp ah, bl
	je pb_validbridge
	cmp ah, bh
	je pb_validbridge
	jmp pb_nextiter  #; not the one we want
pb_validbridge:
	mov r14b, 1  #; valid bridge!
	mov byte ptr [r13], 1
	call placebridge  #; dx already has what to match
	mov byte ptr [r13], 0
pb_nextiter:
	add r12, 2
	add r13, 1
	cmp byte ptr 0[r12], -1
	jne pb_searchloop
	#; all done, check if better score was achieved
	test r14b, r14b
	jnz pb_done
	call getscore
	lea rdi, best[rip]
	cmp rax, [rdi]
	jle pb_done
	mov [rdi], rax
pb_done:
	pop r14
	pop r13
	pop r12
	pop rbx
	ret

main:
	call readdata
	xor dx, dx
	call placebridge

	lea rdi, fmt[rip]
	lea rsi, best[rip]
	mov rsi, [rsi]
	xor rax, rax
	call printf@PLT

	xor rax, rax
	ret
