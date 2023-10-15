.intel_syntax noprefix
#; modified taken from day10/part2.s so it's more generic

.data
	#; TODO we should probably use local variables here
	values: .zero 256
	lengths: .zero 64

.text
	.global knothash

#; rdi -> memory pointer to the sequence of lengths
#; rsi -> number of lengths to take (below 59)
#; rdx -> memory pointer to the output 16 bytes
knothash:
	push rbx  #; rbx -> meory pointer to values
	push rdx  #; we'll be needing dh/dl later on

	#; copy input lengths to our "local" ones
	#; so that we can append the extra values
	push rsi
	mov rcx, rsi
	mov rsi, rdi  #; source = original pointer
	lea rdi, lengths[rip]  #; destination = ours
	rep movsb
	pop rsi
	#; always add `17, 31, 73, 47, 23' to the end
	#; TODO is this right?
	mov byte ptr 0[rdi], 17
	mov byte ptr 1[rdi], 31
	mov byte ptr 2[rdi], 73
	mov byte ptr 3[rdi], 47
	mov byte ptr 4[rdi], 23
	add rsi, 5  #; 5 extra lengths
	#; now the lengths pointer points to our lengths
	lea rdi, lengths[rip]

	#; initialize values
	lea rbx, values[rip]
	mov byte ptr [rbx], 0
	mov rcx, 255
valuesloop:
	mov [rbx+rcx], cl
	loop valuesloop

				  #; rbx -> memory pointer to values
	xor rdx, rdx  #; dh read length, dl swap iters left
	xor r8, r8    #; r8b left tmp index while reversing
	xor r9, r9    #; r9b right tmp index while reversing
	xor rax, rax  #; ah skip value, al index to -> values
	              #; cl tmp hold memory value on swapping
	mov r10, 64   #; number of rounds
roundloop:
	xor r11, r11  #; r11 holds index to length items
	workloop:
		mov r8b, al
		mov r9b, al
		mov dl, [rdi+r11]  #; can't use dh directly
		add r9b, dl
		dec r9b  #; indices start at 0
		mov dh, dl
		shr dl  #; half iterations to swap
		jz revloopout  #; or none if length <= 1
		revloop:
			mov cl, [rbx+r8]
			xchg [rbx+r9], cl
			mov [rbx+r8], cl
			inc r8b
			dec r9b
			dec dl
			jnz revloop
	revloopout:
		add al, dh  #; move the index by length
		add al, ah  #; and also add the skip val
		inc ah      #; larger skip value now
		inc r11     #; pick next length
		cmp r11, rsi
		jne workloop
	#; all length chosen, possibly start another round
	dec r10
	jnz roundloop

	#; all rounds finished, now collapse the values
	#; rbx -> values index
	#; rdx -> output index
	pop rdx
	mov ch, 16
collapseloop:
	mov al, [rbx]
	inc rbx
	mov cl, 15
	collapseinner:
		xor al, [rbx]
		inc rbx
		dec cl
		jnz collapseinner
	mov [rdx], al
	inc rdx
	dec ch
	jnz collapseloop

	pop rbx
	ret
