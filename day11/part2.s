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
	push r15  #; best value

	xor r12, r12
	xor r13, r13
	xor r15, r15

readloop:
	call getchar@PLT
	cmp al, '\n'
	je show
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
	jmp show  #; should not happen

isnorth:
	cmp al, 'w'
	je isnorthwest
	cmp al, 'e'
	je isnortheast
	#; is north (nothing else)
	inc r13
	jmp checklen
isnorthwest:
	dec r12
	inc r13
	jmp checklen
isnortheast:
	inc r12
	jmp checklen

issouth:
	cmp al, 'w'
	je issouthwest
	cmp al, 'e'
	je issoutheast
	#; is south (nothing else)
	dec r13
	jmp checklen
issouthwest:
	dec r12
	jmp checklen
issoutheast:
	inc r12
	dec r13
	jmp checklen

checklen:
	#; calculate the right distance (do NOT modify r12/r13)
	mov rcx, r12
	mov rdx, r13
	cmp rcx, 0
	jl xneg
	cmp rdx, 0
	jl diffsign
	jmp samesign

xneg:
	cmp rdx, 0
	jge diffsign

samesign:
	#; if sign(x) == sign(y) -> abs(x + y)
	mov rax, rcx
	add rax, rdx
	cmp rax, 0
	jge savebest
	neg rax
	jmp savebest

diffsign:
	#; if sign(x) != sign(y) -> max(abs(x), abs(y))
	cmp rcx, 0
	jge nonegx
	neg rcx
nonegx:
	cmp rdx, 0
	jge nonegy
	neg rdx
nonegy:
	cmp rcx, rdx
	jl r13max
	mov rax, rcx
	jmp savebest
r13max:
	mov rax, rdx

savebest:
	#; if we got a better mark, save it
	cmp rax, r15
	cmovg r15, rax
	jmp readloop

show:
	lea rdi, fmt[rip]
	mov rsi, r15
	xor rax, rax
	call printf@PLT

	pop r15
	pop r14
	pop r13
	pop r12
	xor rax, rax
	ret
