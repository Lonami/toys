.intel_syntax noprefix

.data
	.set N, 100  #; amount of scanners
	ranges: .zero N  #; the range of scanner[i]

	fmt: .string "%d\n"
	readn: .string "%d: %d"
	leftn: .zero 8
	rightn: .zero 8

.text
	.global main

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

	xor r9, r9  #; current delay
startwalk:
	xor r8, r8  #; current index
walkloop:
	xor rcx, rcx
	mov cl, [r12+r8]
	test cl, cl
	jz walkloopnext  #; no range, nothing to do
	dec rcx
	shl rcx
	#; the position is current time % ((range - 1) * 2), e.g.
	#; 2 -> 2;    3 -> 4;    4 -> 6;    5 -> 8;    and so on.
	xor rdx, rdx
	mov rax, r8  #; current time = index + delay
	add rax, r9
	div rcx
	#; we have modulo in rdx now.
	#; we actually don't need to do X = (mod - reminder) if reminder > range
	#; to know its exact position, if not, it's outside the real range mod.
	#; we just need to know if it's 0.
	test rdx, rdx
	jnz walkloopnext  #; if it's not zero, we weren't caught
	#; otherwise we were caught and need to restart
	inc r9
	jmp startwalk

walkloopnext:
	inc r8
	cmp r8, 100
	jne walkloop

show:
	lea rdi, fmt[rip]
	mov rsi, r9
	xor rax, rax
	call printf@PLT

	pop r12
	xor rax, rax
	ret
