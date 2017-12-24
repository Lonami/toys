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


#; dl -> bridge to match to
placebridge:
	push rbx
	push r12
	push r13
	lea r12, bridges[rip]  #; r12 = bridges base
	lea r13, used[rip]  #; r13 = used base
	mov bl, dl  #; bl = target
	xor bh, bh  #; bh = flag (any valid)
pb_searchloop:
	cmp byte ptr 0[r12], bl
	jne pb_nextiter  #; not the one we want
	test byte ptr [r13], 0xff
	jnz pb_nextiter  #; in use
	mov bh, 1  #; valid bridge!
	mov byte ptr [r13], 1
	mov dl, byte ptr 1[r12]
	call placebridge
	mov byte ptr [r13], 0
pb_nextiter:
	add r12, 2
	add r13, 1
	mov al, 0[r12]
	nop
	cmp byte ptr 0[r12], -1
	jne pb_searchloop
	#; all done, check if better score was achieved
	test bh, bh
	jnz pb_done
	call getscore
	lea rdi, best[rip]
	cmp rax, [rdi]
	jle pb_done
	mov [rdi], rax
pb_done:
	pop r13
	pop r12
	pop rbx
	ret

main:
	call readdata
	mov dl, 0
	call placebridge

	lea rdi, fmt[rip]
	lea rsi, best[rip]
	mov rsi, [rsi]
	xor rax, rax
	call printf@PLT

	xor rax, rax
	ret
