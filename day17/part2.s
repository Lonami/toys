.intel_syntax noprefix

.data
	.set INPUT, 369
	.set SIZE, 50000000  #; size of the circular buffer array

	fmt: .string "%d\n"

.text
	.global main

main:
	mov rax, 0  #; rax holds position
	mov r8, 1  #; r8 holds next value (& size)
	mov rcx, SIZE-1  #; rcx holds total iterations to do
	mov rsi, 1  #; r10 holds the latest number inserted at position 1
mainloop:
	test rax, rax
	cmovz rsi, r8  #; rax = 0 (which doesn't move) -> insert at 1, new val!
	#; no need to actually insert anything though, just, this is our pos
	inc r8  #; size (and next value) now changed

	add rax, INPUT+1  #; walk by INPUT (+1, we "inserted" at next position)
	xor rdx, rdx
	div r8
	mov rax, rdx    #; modulo current size

	loop mainloop
	lea rdi, fmt[rip]
	#; rsi already has the value
	xor rax, rax
	call printf@PLT

	xor rax, rax
	ret
