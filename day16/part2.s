.intel_syntax noprefix

.data
	readsingle: .string "%ld"
	readpair: .string "%ld/%ld"
	left: .zero 8
	right: .zero 8

	.set N, 16
	.set TIMES, 1000000000
	programs: .ascii "abcdefghijklmnop"
	tmparray: .space N  #; spinning

	#; since we're gonna need to iterate over the input many times, save
	#; the processed input in memory, already parsed indices -> no scanf.
	input: .zero 65536

.text
	.global main


#; rbx -> base of [input], it will increase
readinput:
ri_loop:
	call getchar@PLT
	cmp al, 's'
	je ri_spin
	cmp al, 'x'
	je ri_exchange
	#;cmp al, 'p'
	#;je ri_partner  #; assume well formed input
ri_partner:
	mov byte ptr [rbx], 'p'
	inc rbx
	call getchar@PLT
	mov [rbx], al
	inc rbx
	call getchar@PLT  #; '/'
	call getchar@PLT
	mov [rbx], al
	inc rbx
	jmp ri_nextiter
ri_spin:
	mov byte ptr [rbx], 's'
	inc rbx
	lea rdi, readsingle[rip]
	lea rsi, left[rip]
	xor rax, rax
	call scanf@PLT
	mov rax, left[rip]
	mov [rbx], al
	inc rbx
	jmp ri_nextiter
ri_exchange:
	mov byte ptr [rbx], 'x'
	inc rbx
	lea rdi, readpair[rip]
	lea rsi, left[rip]
	lea rdx, right[rip]
	xor rax, rax
	call scanf@PLT
	mov rax, left[rip]
	mov [rbx], al
	inc rbx
	mov rax, right[rip]
	mov [rbx], al
	inc rbx
ri_nextiter:
	call getchar@PLT
	cmp al, ','
	je ri_loop
	ret


main:
	push rbx
	push r12

	lea rbx, input[rip]
	call readinput

	#; now our 'input' is parsed so we don't have to call getchar/scanf.
	#; we will also be using 'lodsb' that is equivalent to 'al = [rsi++]'
	#; first cleanup the high parts of rax/rdx, and setup the programs base
	xor rax, rax
	xor rdx, rdx
	lea rbx, programs[rip]
	mov r8, TIMES

mainloop:
	#; restart our input moving pointer
	lea rsi, input[rip]

	danceloop:
		lodsb
		cmp al, 's'
		je spin
		cmp al, 'x'
		je exchange
		cmp al, 'p'
		je partner
		#; we reached a '\0'
		dec r8
		jz done
		jmp mainloop

	partner:
		lodsb
		lea rdi, programs[rip]
		mov rcx, N
		repne scasb
		mov r12, rdi  #; first partner pos in r12
		lodsb
		lea rdi, programs[rip]
		mov rcx, N
		repne scasb  #; second partner pos in rdi
		#; xcgh -1[rdi], -1[r12], as it overshoots
		mov cl, -1[rdi]
		xchg -1[r12], cl
		mov -1[rdi], cl
		jmp danceloop
	spin:
		lodsb
		mov r12, rsi  #; save rsi in r12 temporarily
		lea rsi, programs+N[rip]
		sub rsi, rax
		lea rdi, tmparray[rip]
		mov rcx, rax
		rep movsb
		lea rsi, programs[rip]
		mov rcx, N
		sub rcx, rax
		rep movsb

		lea rsi, tmparray[rip]
		lea rdi, programs[rip]
		mov rcx, N
		rep movsb
		mov rsi, r12  #; restore r12
		jmp danceloop
	exchange:
		lodsb
		mov dl, al
		lodsb
		mov cl, [rbx+rax]
		xchg [rbx+rdx], cl
		mov [rbx+rax], cl
		jmp danceloop

done:
	xor r12, r12
showloop:
	mov rdi, [rbx+r12]
	call putchar@PLT
	inc r12
	cmp r12, N
	jne showloop
	mov rdi, '\n'
	call putchar@PLT

	pop r12
	pop rbx
	xor rax, rax
	ret
