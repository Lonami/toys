.intel_syntax noprefix

.data
	pointfmt: .string "p=<%ld,%ld,%ld>, v=<%ld,%ld,%ld>, a=<%ld,%ld,%ld> "
	pointread: .space 9*8

	fmt: .string "%d\n"

	.set MAXPOINTS, 1000
	points: .zero MAXPOINTS*9*8

.text
	.global main


readinput:
	push rbx
	push r12
	mov rbx, rdi
	xor r12, r12
ri_loop:
	lea rdi, pointfmt[rip]
	lea rsi, pointread   [rip]
	lea rdx, pointread+ 8[rip]
	lea rcx, pointread+16[rip]
	lea  r8, pointread+24[rip]
	lea  r9, pointread+32[rip]
	lea rax, pointread+64[rip]
	push rax
	lea rax, pointread+56[rip]
	push rax
	lea rax, pointread+48[rip]
	push rax
	lea rax, pointread+40[rip]
	push rax
	xor rax, rax
	call scanf@PLT
	add rsp, 32
	cmp rax, 9
	jne ri_done
	inc r12
	mov rcx, 9
	lea rsi, pointread[rip]
	mov rdi, rbx
	rep movsq
	mov rbx, rdi
	jmp ri_loop
ri_done:
	mov rax, r12
	pop r12
	pop rbx
	ret


#; rdi -> [points] base
#; rsi -> number of elements
simulate:
	mov rcx, rsi
s_loop:
	#; vel += acc
	mov  r8, 48[rdi]
	mov  r9, 56[rdi]
	mov r10, 64[rdi] 
	add 24[rdi], r8
	add 32[rdi], r9
	add 40[rdi], r10
	#; pos += vel
	mov  r8, 24[rdi]
	mov  r9, 32[rdi]
	mov r10, 40[rdi] 
	add  0[rdi], r8
	add  8[rdi], r9
	add 16[rdi], r10
	add rdi, 9*8
	loop s_loop
	ret


main:
	push r12
	push r13

	lea rdi, points[rip]
	call readinput
	mov r12, rax  #; r12 number of points

	#; the one that will stay closest to 0 is the one
	#; with SMALLEST acceleration TOWARDS the zero
	#; based on their POSITION. velocity is irrelevant
	#;
	#; ...or we can just run the simulation a thousand times
	mov r13, 1000
mainloop:
	lea rdi, points[rip]
	mov rsi, r12
	call simulate
	dec r13
	jnz mainloop

	lea rdi, points[rip]
	mov r9, 0x7fffffffffffffff  #; closest distance until now
	xor r10, r10  #; index of the closest distance
	mov rcx, r12
searchloop:
	xor r8, r8  #; current distance
	mov rdx,  0[rdi]
	mov rax, rdx
	neg rax
	cmovl rax, rdx  #; absolute value
	add r8, rax
	mov rdx,  8[rdi]
	mov rax, rdx
	neg rax
	cmovl rax, rdx
	add r8, rax
	mov rdx, 16[rdi]
	mov rax, rdx
	neg rax
	cmovl rax, rdx
	add r8, rax
	cmp r9, r8
	jl searchnext
	mov r9, r8
	mov r10, rcx
searchnext:
	add rdi, 9*8
	loop searchloop

	lea rdi, fmt[rip]
	mov rsi, r12
	sub rsi, r10
	xor rax, rax
	call printf@PLT

	pop r13
	pop r12
	xor rax, rax
	ret
