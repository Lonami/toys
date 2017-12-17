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
	lea rbx, programs[rip]
	#; r8 will count how many dances were taken before returning to original
	xor r8, r8
mainloop:
	#; first cleanup the high parts of rax/rdx, and setup the programs base
	xor rax, rax
	xor rdx, rdx

	#; restart our input moving pointer
	inc r8
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
		jmp checkstate

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

checkstate:
	#; check if we've gotten back to 'abcd...'. if we have we can
	#; use some modular math to only do up to there (thanks udf!)
	mov rcx, N-1
	lea rsi, programs[rip]
	lea rdi, programs+1[rip]
cs_loop:
	mov ah, [rsi]
	mov al, [rdi]
	sub al, ah
	cmp al, 1
	jne mainloop  #; some difference wasn't 1, so they're unordered, next iter
	inc rsi
	inc rdi
	loop cs_loop

foundstart:
	#; r8 now contains the number of iterations we need to take, yay!
	#; now we only need to do TIMES % r8, and modulo will be amount of
	#; times to run this
	xor rdx, rdx
	mov rax, TIMES
	div r8
	mov r8, rdx

	#; TODO don't duplicate code this much
	#; first cleanup the high parts of rax/rdx, and setup the programs base
	xor rax, rax
	xor rdx, rdx
mainloop2:
	#; restart our input moving pointer
	lea rsi, input[rip]

	danceloop2:
		lodsb
		cmp al, 's'
		je spin2
		cmp al, 'x'
		je exchange2
		cmp al, 'p'
		je partner2
		#; we reached a '\0'
		dec r8
		jz done
		jmp mainloop2

	partner2:
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
		jmp danceloop2
	spin2:
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
		jmp danceloop2
	exchange2:
		lodsb
		mov dl, al
		lodsb
		mov cl, [rbx+rax]
		xchg [rbx+rdx], cl
		mov [rbx+rax], cl
		jmp danceloop2


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
