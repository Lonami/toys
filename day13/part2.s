.intel_syntax noprefix

.data
	.set N, 100  #; amount of scanners
	ranges: .zero N  #; the range of scanner[i]
	scanners: .zero N  #; the current position of scanners
	updown: .space N, 1  #; moving up (+1) or down (-1), flipped on bounds

	fmt: .string "%d\n"
	readn: .string "%d: %d"
	leftn: .zero 8
	rightn: .zero 8

.text
	.global main

#; r12 points to the base of ranges
#; rsi points to the base of scanners
#; rdi points to the base of updown
#; uses r9, r10
movescanners:
	mov rcx, N
ms_loop:
	mov r9b, -1[r12+rcx]   #; r9b  = range - 1
	mov r10b, -1[rsi+rcx]  #; r10b = scanner position
	test r9b, r9b
	jz ms_nextiter  #; important: no range, do nothing
	dec r9b
	add r10b, -1[rdi+rcx]  #; moving up or down?
	#; check bounds and swap if necessary
	cmp r10b, r9b
	je ms_swapdir
	test r10b, r10b
	jz ms_swapdir
	jmp ms_nextiter
ms_swapdir:
	neg byte ptr -1[rdi+rcx]  #; swap direction in memory
ms_nextiter:
	mov -1[rsi+rcx], r10b  #; save new position
	loop ms_loop
	ret


main:
	push r12

	lea r12, ranges[rip]
readloop:
	lea rdi, readn[rip]
	lea rsi, leftn[rip]
	lea rdx, rightn[rip]
	xor rax, rax
	call scanf@PLT
	cmp al, -1
	je readdone
	mov rax, leftn[rip]
	mov rdx, rightn[rip]
	mov [r12+rax], dl
	jmp readloop

readdone:
	lea rsi, scanners[rip]
	lea rdi, updown[rip]

	xor rax, rax  #; current delay
startwalk:
	xor r8, r8  #; current index
walkloop:
	#; we walk on the 0'th line, so if it's not zero scanner isn't here
	test byte ptr [rsi+r8], 0xff
	jz caught
fakecaught:
	call movescanners
	inc r8
	cmp r8, 100
	je show  #; we completed it and didn't get caught!

caught:
	#; we might not be caught, simply there was no range
	test byte ptr [r12+r8], 0xff
	jz fakecaught
	#; there was a range, we really got caught, incremenet skip
	inc rax
	#; reset scanners and their sign
	mov rcx, N
caughtclean:
	#; could rep stosb but we need to save in two positions anyway
	mov byte ptr -1[rsi+rcx], 0
	mov byte ptr -1[rdi+rcx], 1
	loop caughtclean
	#; advance scanners delay times (rcx is used by movescanners)
	mov rdx, rax
caughtdelay:
	call movescanners
	dec rdx
	jnz caughtdelay
	jmp startwalk

show:
	lea rdi, fmt[rip]
	mov rsi, rax
	xor rax, rax
	call printf@PLT

	pop r12
	xor rax, rax
	ret
