.intel_syntax noprefix

.data
	input: .zero 16
	hashed: .space 16

	fmt: .string "%d\n"

.text
	.global main


#; rdi points to where to start appending
#; r13 has the number to append to input
#; in rax returns how big it was
appendint:
	xor rcx, rcx  #; how many items were pushed
	mov rax, r13  #; rax will keep r13 while decreasing
	mov r8, 10    #; constant 10

ai_loop:
	cmp rax, r8
	jl ai_done
	xor rdx, rdx
	div r8
	inc rcx   #; one more digit
	push rdx  #; the stack will hold our digits
	jmp ai_loop

ai_done:
	inc rcx
	push rax  #; last digit

	#; now we need to append it to the input
	mov rax, rcx  #; in rax we return the amount of digits added
ai_append:
	pop rdx
	add dl, '0'  #; we want ascii chars
	mov [rdi], dl
	inc rdi
	loop ai_append
	ret


main:
	push rbx
	push r12  #; input length
	push r13  #; index from 0..127
	push r14  #; total count of bits
	lea rbx, input[rip]

readloop:
	call getchar@PLT
	cmp al, '\n'
	je readdone
	mov [rbx], al
	inc rbx
	jmp readloop
readdone:
	mov byte ptr [rbx], '-'
	inc rbx
	mov r12, rbx
	lea rbx, input[rip]
	sub r12, rbx

	xor r13, r13  #; index from 0..127
	xor r14, r14  #; total count of bits
hashloop:
	lea rdi, input[rip]
	add rdi, r12
	call appendint

	lea rdi, input[rip]
	mov rsi, r12
	add rsi, rax  #; result from appendint
	lea rdx, hashed[rip]
	call knothash

	#; count the number of bits set
	mov rcx, 16
	lea rdx, hashed[rip]
countbits:
	mov al, -1[rdx+rcx]
	countbitsinner:
		test al, al
		jz countbitsnext
		test al, 1
		jz countbitsskip
		inc r14
		countbitsskip:
		shr al
		jmp countbitsinner
	countbitsnext:
	loop countbits

	inc r13
	cmp r13, 128
	jne hashloop

	lea rdi, fmt[rip]
	mov rsi, r14
	xor rax, rax
	call printf@PLT

	pop r14
	pop r13
	pop r12
	pop rbx
	xor rax, rax
	ret
