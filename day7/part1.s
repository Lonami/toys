.intel_syntax noprefix

.data
	#; two arrays, one will hold names, the other weights
	#; thanksfully, names are 7 chars or less, so they fit
	#; in a single register!
	.set N, 2048  #; maximum number of lines, not actual lines
	names: .zero N*8  #; tower names
	weights: .zero N*8  #; tower weights

	#; now to store who holds who, we have a matrix
	#; [[who, holds...]], N rows of max M-1 holds of 8 size
	.set HOLDERS, 16
	holders: .zero 512*HOLDERS*8

	#; scanf can read the '(weight)' for us just fine
	weightfmt: .string "(%ld)"
	tempn: .quad

.text
	.global main

#; reads a name and puts it in rax
#; rbx holds the last character read that caused the exit
readname:
	xor rbx, rbx
readnameloop:
	call getchar@PLT
	cmp al, ' '
	je readnamedone
	cmp al, '\n'
	je readnamedone
	cmp al, ','
	je readnamedone

	shl rbx, 8
	mov bl, al
	jmp readnameloop
readnamedone:
	xchg rax, rbx
	ret


#; shows a name on screen, must be in rax
showname:
	push rax
	push rbx
	mov rbx, rax

shownameloop:
	rol rbx, 8
	test bl, bl
	jz shownamenext
	movzbq rdi, bl
	xor bl, bl
	call putchar@PLT

shownamenext:
	test rbx, rbx
	jnz shownameloop
	mov rdi, '\n'
	call putchar@PLT

	pop rbx
	pop rax
	ret


main:
	push rbp
	push r12
	push r13
	push r14
	push r15

	lea r12, names[rip]
	lea r13, weights[rip]
	lea r14, holders[rip]
readloop:
	call readname
	test rax, rax
	jz readdone  #; simply a new line (empty name) means done

	mov rbp, rax  #; save original name in rbp for later
	mov [r12], rax
	lea rdi, weightfmt[rip]
	lea rsi, tempn[rip]
	mov rax, 0
	call scanf@PLT
	mov rax, tempn[rip]
	mov [r13], rax

	call getchar@PLT
	cmp al, '\n'
	je readnext
	#; else, we read a space, then comes '-> '
	call getchar@PLT
	call getchar@PLT
	call getchar@PLT

	#; original name in rbp, so we know the holder
	#; r15 will contain the size of the added holders
	mov [r14], rbp
	add r14, 8
	mov r15, 8
readholders:
	call readname

	#; always save the name
	mov [r14], rax
	add r14, 8
	add r15, 8

	#; check if we read a newline, then we're done
	cmp bl, '\n'
	je readholdersdone

	#; else bl = ',' (read the space so to consume the whole ', ')
	call getchar@PLT
	jmp readholders

readholdersdone:
	#; next row of holders
	sub r14, r15
	add r14, HOLDERS*8

readnext:
	add r12, 8
	add r13, 8
	jmp readloop
readdone:

	#; now that we've finally read all data,
	#; we need to find who's at the bottom. but who?
	#; well, simply the one who's not held by anyone!
	lea r12, names[rip]

	#; r12 will be the iterator over names
	#; r14 will be the iterator of ROWS over holders
findbottom_loop:
	mov rax, [r12]  #; with this name, search in holders
	add r12, 8
	lea r14, holders[rip]  #; start again for every name

findbottom_rowloop:
	mov rcx, [r14]
	add r14, HOLDERS*8
	test rcx, rcx
	jz found_bottom  #; no holders in clean row, end!

	mov rbx, 0  #; index inside current holders row
findbottom_holderloop:
	inc rbx
	mov rcx, [r14+rbx*8]
	test rcx, rcx
	jz findbottom_rowloop  #; no more holders here, next row

	cmp rcx, rax
	je findbottom_loop  #; we found someone holding us
	jmp findbottom_holderloop  #; check more holders

found_bottom:
	call showname

	pop r15
	pop r14
	pop r13
	pop r12
	pop rbp
	xor rax, rax
	ret

