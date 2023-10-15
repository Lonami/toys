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


#; rsi -> base of [input], will increase
#; rbx -> base of [programs]
#; uses rax, rcx, rdx, r8
dance:
	xor rax, rax  #; cleanup high parts
	xor rdx, rdx
d_loop:
	lodsb
	cmp al, 's'
	je d_spin
	cmp al, 'x'
	je d_exchange
	cmp al, 'p'
	je d_partner
	ret  #; we reached a '\0'
d_partner:
	lodsb
	lea rdi, programs[rip]
	mov rcx, N
	repne scasb
	mov r8, rdi  #; first partner pos in r8
	lodsb
	lea rdi, programs[rip]
	mov rcx, N
	repne scasb  #; second partner pos in rdi
	#; xcgh -1[rdi], -1[r8], as it overshoots
	mov cl, -1[rdi]
	xchg -1[r8], cl
	mov -1[rdi], cl
	jmp d_loop
d_spin:
	lodsb
	mov r8, rsi  #; save rsi in r8 temporarily
	lea rsi, programs+N[rip]
	sub rsi, rax
	lea rdi, tmparray[rip]
	mov rcx, rax
	rep movsb
	lea rsi, programs[rip]
	mov rcx, N
	sub rcx, rax
	rep movsb
	#; now copy back tmparray -> programs
	lea rsi, tmparray[rip]
	lea rdi, programs[rip]
	mov rcx, N
	rep movsb
	mov rsi, r8  #; restore r8
	jmp d_loop
d_exchange:
	lodsb
	mov dl, al
	lodsb
	mov cl, [rbx+rax]
	xchg [rbx+rdx], cl
	mov [rbx+rax], cl
	jmp d_loop
	#; ret is above


main:
	push rbx
	push r12

	lea rbx, input[rip]
	call readinput

	#; now our 'input' is parsed so we don't have to call getchar/scanf.
	#; we will also be using 'lodsb' that is equivalent to 'al = [rsi++]'
	#; r9 will count how many dances were taken before returning to original
	xor r9, r9
	lea rbx, programs[rip]
findrepeat:
	#; first cleanup the high parts of rax/rdx, and setup the programs base
	inc r9
	lea rsi, input[rip]
	call dance
checkstate:
	#; check if we've gotten back to 'abcd...'. if we have we can
	#; use some modular math to only do up to there (thanks udf!)
	mov rcx, N-1
	lea rsi, programs[rip]
	lea rdi, programs+1[rip]
checkloop:
	mov ah, [rsi]
	mov al, [rdi]
	sub al, ah
	cmp al, 1
	jne findrepeat  #; some difference wasn't 1, so they're unordered
	inc rsi
	inc rdi
	loop checkloop

foundstart:
	#; r9 now contains the number of iterations we need to take, yay!
	#; now we only need to do TIMES % r9, and modulo will be amount of
	#; times to run this
	xor rdx, rdx
	mov rax, TIMES
	div r9
	mov r9, rdx
danceloop:
	lea rsi, input[rip]
	call dance
	dec r9
	jnz danceloop

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
