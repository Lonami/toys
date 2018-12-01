.intel_syntax noprefix

.data
	fmt: .string "%d\n"

.text
	.global main

main:
	#; stackoverflow.com/a/5085274/4759433
	push r12  #; horizontal
	push r13  #; vertical
	push r14  #; read value

	xor r12, r12
	xor r13, r13

readloop:
	call getchar@PLT
	cmp al, '\n'
	je done
	mov r14b, al
	shl r14, 8
	call getchar@PLT
	mov r14b, al
	cmp al, ','
	je readitem
	call getchar@PLT

readitem:
	mov ax, r14w
	cmp ah, 'n'
	je isnorth
	cmp ah, 's'
	je issouth
	jmp done  #; should not happen

isnorth:
	cmp al, 'w'
	je isnorthwest
	cmp al, 'e'
	je isnortheast
	#; is north (nothing else)
	inc r13
	jmp readloop
isnorthwest:
	dec r12
	inc r13
	jmp readloop
isnortheast:
	inc r12
	jmp readloop

issouth:
	cmp al, 'w'
	je issouthwest
	cmp al, 'e'
	je issoutheast
	#; is south (nothing else)
	dec r13
	jmp readloop
issouthwest:
	dec r12
	jmp readloop
issoutheast:
	inc r12
	dec r13
	jmp readloop

done:
	#; calculate the distance
	cmp r12, 0
	jl xneg
	cmp r13, 0
	jl diffsign
	jmp samesign

xneg:
	cmp r13, 0
	jge diffsign

samesign:
	#; if sign(x) == sign(y) -> abs(x + y)
	mov rax, r12
	add rax, r13
	cmp rax, 0
	jge show
	neg rax
	jmp show

diffsign:
	#; if sign(x) != sign(y) -> max(abs(x), abs(y))
	cmp r12, 0
	jge nonegx
	neg r12
nonegx:
	cmp r13, 0
	jge nonegy
	neg r13
nonegy:
	cmp r12, r13
	jl r13max
	mov rax, r12
	jmp show
r13max:
	mov rax, r13

show:
	lea rdi, fmt[rip]
	mov rsi, rax
	xor rax, rax
	call printf@PLT

	pop r14
	pop r13
	pop r12
	xor rax, rax
	ret
