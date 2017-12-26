.intel_syntax noprefix

.data
	pointfmt: .string "p=<%ld,%ld,%ld>, v=<%ld,%ld,%ld>, a=<%ld,%ld,%ld> "
	pointread: .space 9*8

	fmt: .string "%d\n"

	.set MAXPOINTS, 1000
	points: .zero MAXPOINTS*9*8
	destroyed: .zero MAXPOINTS

.text
	.global main


#; rdi -> [points] base
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


#; rdi -> [destroyed] base
#; rsi -> [points] base
#; rdx -> number of elements
simulate:
	mov rcx, rdx
s_loop:
	test byte ptr [rdi], 1
	jnz s_next
	#; vel += acc
	mov  r8, 48[rsi]
	mov  r9, 56[rsi]
	mov r10, 64[rsi]
	add 24[rsi], r8
	add 32[rsi], r9
	add 40[rsi], r10
	#; pos += vel
	mov  r8, 24[rsi]
	mov  r9, 32[rsi]
	mov r10, 40[rsi]
	add  0[rsi], r8
	add  8[rsi], r9
	add 16[rsi], r10
s_next:
	inc rdi
	add rsi, 9*8
	loop s_loop
	ret
#; dprintf s_next, "part[%d].pos = (%ld, %ld, %ld); dead? %c\n", $rdx-$rcx, *$rsi, *($rsi+8), *($rsi+16), (*$rdi)+'0'


#; rdi -> [destroyed] base
#; rsi -> [points] base
#; rdx -> number of elements
checkcoll:
	push rbx
	lea rcx, -1[rdx]  #; count down, never last item
cc_loop:
	test byte ptr [rdi], 1
	jnz cc_nextiter
	mov  r8,  0[rsi]
	mov  r9,  8[rsi]
	mov r10, 16[rsi]
	mov rax, rsi  #; rax = inner access
	mov rdx, rdi  #; rdx = flag destination
	mov r11, rcx  #; r11 = inner data count (-1 before +1 now -> reaching end)
	xor bl, bl  #; bl = flag if any was destroyed
	cc_inloop:
		inc rdx
		add rax, 9*8
		test byte ptr [rdx], 1
		jnz cc_nextin
		cmp  r8,  0[rax]
		jne cc_nextin
		cmp  r9,  8[rax]
		jne cc_nextin
		cmp r10, 16[rax]
		jne cc_nextin
		mov bl, 1
		mov byte ptr [rdx], 1
	cc_nextin:
		dec r11
		jnz cc_inloop
	test bl, bl
	jz cc_nextiter
	mov byte ptr [rdi], 1
cc_nextiter:
	add rdi, 1
	add rsi, 9*8
	loop cc_loop
	pop rbx
	ret


main:
	push r12
	push r13

	lea rdi, points[rip]
	call readinput
	mov r12, rax  #; r12 number of points

	#; run the simulation a few times to resolve all collisions
	mov r13, 1000
mainloop:
	lea rdi, destroyed[rip]
	lea rsi, points[rip]
	mov rdx, r12
	call simulate

	lea rdi, destroyed[rip]
	lea rsi, points[rip]
	mov rdx, r12
	call checkcoll

	dec r13
	jnz mainloop

	xor rsi, rsi  #; keep collision count here
	lea rdi, destroyed[rip]
	mov rcx, r12
countloop:
	test byte ptr -1[rdi+rcx], 1
	jnz countnext
	inc rsi
countnext:
	loop countloop

	lea rdi, fmt[rip]
	#; rsi has count
	xor rax, rax
	call printf@PLT

	pop r13
	pop r12
	xor rax, rax
	ret
