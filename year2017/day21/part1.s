.intel_syntax noprefix

.data
	.set MAXR2, 8
	.set MAXR3, 128
	.set N, 1024  #; board dimensions

	fmt: .string "%d\n"

	#; 0 means off, 1 means on, -1 means not set
	rules2: .space MAXR2*13, -1  #; 2*2 + 3*3 = 4 +  9
	rules3: .space MAXR3*25, -1  #; 3*3 + 4*4 = 9 + 16

	#; swap between board1 and board2 not to overwrite while applying rules
	board1: .zero N*N
	board2: .zero N*N

.text
	.global main


#; rdi -> [rules2] base
#; rsi -> [rules3] base
readrules:
	push rbx
	push r12
	push r13
	mov r12, rdi
	mov r13, rsi
rr_loop:
	call getchar@PLT
	cmp al, '\n'
	jle rr_done
	#; start by assuming it's a 3x3 rule
	cmp al, '#'
	sete 0[r13]
	call getchar@PLT
	cmp al, '#'
	sete 1[r13]
	call getchar@PLT
	cmp al, '/'
	je rr_rule2
rr_rule3:
	cmp al, '#'
	sete 2[r13]
	add r13, 3
rr_rule3loop:
	call getchar@PLT
	cmp al, '\n'
	je rr_loop
	cmp al, '.'
	je rr_rule3apply
	cmp al, '#'
	jne rr_rule3loop
rr_rule3apply:
	cmp al, '#'
	sete [r13]
	inc r13
	jmp rr_rule3loop

rr_rule2:
	#; need to copy from rule3 -> rule2
	mov al, 0[r13]
	mov 0[r12], al
	mov byte ptr 0[r13], -1
	mov al, 1[r13]
	mov 1[r12], al
	mov byte ptr 1[r13], -1
	add r12, 2
rr_rule2loop:
	call getchar@PLT
	cmp al, '\n'
	je rr_loop
	cmp al, '.'
	je rr_rule2apply
	cmp al, '#'
	jne rr_rule2loop
rr_rule2apply:
	cmp al, '#'
	sete [r12]
	inc r12
	jmp rr_rule2loop

rr_done:
	pop r13
	pop r12
	pop rbx
	ret


#; rdi -> [rules2] base
#; rsi -> [top left corner] base
#; rax <- memory location of the matching rule
findmatch2:
	#; load the pattern from rsi as follows:
	#;  cl ch
	#;  dl dh
	mov cl,   0[rsi]
	mov ch,   1[rsi]
	mov dl, N+0[rsi]
	mov dh, N+1[rsi]
fm2_loop:
fm2_checknormal:
	cmp cl, 0[rdi]
	jne fm2_checkflipy
	cmp ch, 1[rdi]
	jne fm2_checkflipy
	cmp dl, 2[rdi]
	jne fm2_checkflipy
	cmp dh, 3[rdi]
	jne fm2_checkflipy
	jmp fm2_done
fm2_checkflipy:
	cmp cl, 2[rdi]
	jne fm2_checkflipx
	cmp ch, 3[rdi]
	jne fm2_checkflipx
	cmp dl, 0[rdi]
	jne fm2_checkflipx
	cmp dh, 1[rdi]
	jne fm2_checkflipx
	jmp fm2_done
fm2_checkflipx:
	cmp cl, 1[rdi]
	jne fm2_checkflipxrot90
	cmp ch, 0[rdi]
	jne fm2_checkflipxrot90
	cmp dl, 3[rdi]
	jne fm2_checkflipxrot90
	cmp dh, 2[rdi]
	jne fm2_checkflipxrot90
	jmp fm2_done
fm2_checkflipxrot90:
	cmp cl, 3[rdi]
	jne fm2_checkflipxrot270
	cmp ch, 1[rdi]
	jne fm2_checkflipxrot270
	cmp dl, 2[rdi]
	jne fm2_checkflipxrot270
	cmp dh, 0[rdi]
	jne fm2_checkflipxrot270
	jmp fm2_done
fm2_checkflipxrot270:
	cmp cl, 0[rdi]
	jne fm2_checkrot90
	cmp ch, 2[rdi]
	jne fm2_checkrot90
	cmp dl, 1[rdi]
	jne fm2_checkrot90
	cmp dh, 3[rdi]
	jne fm2_checkrot90
	jmp fm2_done
fm2_checkrot90:
	cmp cl, 2[rdi]
	jne fm2_checkrot180
	cmp ch, 0[rdi]
	jne fm2_checkrot180
	cmp dl, 3[rdi]
	jne fm2_checkrot180
	cmp dh, 1[rdi]
	jne fm2_checkrot180
	jmp fm2_done
fm2_checkrot180:
	cmp cl, 3[rdi]
	jne fm2_checkrot270
	cmp ch, 2[rdi]
	jne fm2_checkrot270
	cmp dl, 1[rdi]
	jne fm2_checkrot270
	cmp dh, 0[rdi]
	jne fm2_checkrot270
	jmp fm2_done
fm2_checkrot270:
	cmp cl, 1[rdi]
	jne fm2_fail
	cmp ch, 3[rdi]
	jne fm2_fail
	cmp dl, 0[rdi]
	jne fm2_fail
	cmp dh, 2[rdi]
	jne fm2_fail
	jmp fm2_done
fm2_fail:
	add rdi, 25
	cmp byte ptr [rdi], -1
	jne fm2_loop
fm2_finalfail:
	xor rdi, rdi
fm2_done:
	mov rax, rdi
	ret


#; rdi -> [rules3] base
#; rsi -> [top left corner] base
#; rax <- memory location of the matching rule
findmatch3:
	#; load the pattern from rsi as follows:
	#;  al  ah   cl
	#;  ch  dl   dh
	#; r8b r9b r10b
	mov al,       0[rsi]
	mov ah,       1[rsi]
	mov cl,       2[rsi]
	mov ch,     N+0[rsi]
	mov dl,     N+1[rsi]
	mov dh,     N+2[rsi]
	mov r8b,  2*N+0[rsi]
	mov r9b,  2*N+1[rsi]
	mov r10b, 2*N+2[rsi]
fm3_loop:
fm3_checknormal:
	cmp al, 0[rdi]
	jne fm3_checkflipy
	cmp ah, 1[rdi]
	jne fm3_checkflipy
	cmp cl, 2[rdi]
	jne fm3_checkflipy
	cmp ch, 3[rdi]
	jne fm3_checkflipy
	cmp dl, 4[rdi]
	jne fm3_checkflipy
	cmp dh, 5[rdi]
	jne fm3_checkflipy
	cmp r8b, 6[rdi]
	jne fm3_checkflipy
	cmp r9b, 7[rdi]
	jne fm3_checkflipy
	cmp r10b, 8[rdi]
	jne fm3_checkflipy
	jmp fm3_done
fm3_checkflipy:
	cmp al, 2[rdi]
	jne fm3_checkflipx
	cmp ah, 1[rdi]
	jne fm3_checkflipx
	cmp cl, 0[rdi]
	jne fm3_checkflipx
	cmp ch, 5[rdi]
	jne fm3_checkflipx
	cmp dl, 4[rdi]
	jne fm3_checkflipx
	cmp dh, 3[rdi]
	jne fm3_checkflipx
	cmp r8b, 8[rdi]
	jne fm3_checkflipx
	cmp r9b, 7[rdi]
	jne fm3_checkflipx
	cmp r10b, 6[rdi]
	jne fm3_checkflipx
	jmp fm3_done
fm3_checkflipx:
	cmp al, 6[rdi]
	jne fm3_checkflipxrot90
	cmp ah, 7[rdi]
	jne fm3_checkflipxrot90
	cmp cl, 8[rdi]
	jne fm3_checkflipxrot90
	cmp ch, 3[rdi]
	jne fm3_checkflipxrot90
	cmp dl, 4[rdi]
	jne fm3_checkflipxrot90
	cmp dh, 5[rdi]
	jne fm3_checkflipxrot90
	cmp r8b, 0[rdi]
	jne fm3_checkflipxrot90
	cmp r9b, 1[rdi]
	jne fm3_checkflipxrot90
	cmp r10b, 2[rdi]
	jne fm3_checkflipxrot90
	jmp fm3_done
fm3_checkflipxrot90:
	cmp al, 0[rdi]
	jne fm3_checkflipxrot270
	cmp ah, 3[rdi]
	jne fm3_checkflipxrot270
	cmp cl, 6[rdi]
	jne fm3_checkflipxrot270
	cmp ch, 1[rdi]
	jne fm3_checkflipxrot270
	cmp dl, 4[rdi]
	jne fm3_checkflipxrot270
	cmp dh, 7[rdi]
	jne fm3_checkflipxrot270
	cmp r8b, 2[rdi]
	jne fm3_checkflipxrot270
	cmp r9b, 5[rdi]
	jne fm3_checkflipxrot270
	cmp r10b, 8[rdi]
	jne fm3_checkflipxrot270
	jmp fm3_done
fm3_checkflipxrot270:
	cmp al, 8[rdi]
	jne fm3_checkrot90
	cmp ah, 5[rdi]
	jne fm3_checkrot90
	cmp cl, 2[rdi]
	jne fm3_checkrot90
	cmp ch, 7[rdi]
	jne fm3_checkrot90
	cmp dl, 4[rdi]
	jne fm3_checkrot90
	cmp dh, 1[rdi]
	jne fm3_checkrot90
	cmp r8b, 6[rdi]
	jne fm3_checkrot90
	cmp r9b, 3[rdi]
	jne fm3_checkrot90
	cmp r10b, 0[rdi]
	jne fm3_checkrot90
	jmp fm3_done
fm3_checkrot90:
	cmp al, 6[rdi]
	jne fm3_checkrot180
	cmp ah, 3[rdi]
	jne fm3_checkrot180
	cmp cl, 0[rdi]
	jne fm3_checkrot180
	cmp ch, 7[rdi]
	jne fm3_checkrot180
	cmp dl, 4[rdi]
	jne fm3_checkrot180
	cmp dh, 1[rdi]
	jne fm3_checkrot180
	cmp r8b, 8[rdi]
	jne fm3_checkrot180
	cmp r9b, 5[rdi]
	jne fm3_checkrot180
	cmp r10b, 2[rdi]
	jne fm3_checkrot180
	jmp fm3_done
fm3_checkrot180:
	cmp al, 8[rdi]
	jne fm3_checkrot270
	cmp ah, 7[rdi]
	jne fm3_checkrot270
	cmp cl, 6[rdi]
	jne fm3_checkrot270
	cmp ch, 5[rdi]
	jne fm3_checkrot270
	cmp dl, 4[rdi]
	jne fm3_checkrot270
	cmp dh, 3[rdi]
	jne fm3_checkrot270
	cmp r8b, 2[rdi]
	jne fm3_checkrot270
	cmp r9b, 1[rdi]
	jne fm3_checkrot270
	cmp r10b, 0[rdi]
	jne fm3_checkrot270
	jmp fm3_done
fm3_checkrot270:
	cmp al, 2[rdi]
	jne fm3_fail
	cmp ah, 5[rdi]
	jne fm3_fail
	cmp cl, 8[rdi]
	jne fm3_fail
	cmp ch, 1[rdi]
	jne fm3_fail
	cmp dl, 4[rdi]
	jne fm3_fail
	cmp dh, 7[rdi]
	jne fm3_fail
	cmp r8b, 0[rdi]
	jne fm3_fail
	cmp r9b, 3[rdi]
	jne fm3_fail
	cmp r10b, 6[rdi]
	jne fm3_fail
	jmp fm3_done
fm3_fail:
	add rdi, 25
	cmp byte ptr [rdi], -1
	jne fm3_loop
fm3_finalfail:
	xor rdi, rdi
fm3_done:
	mov rax, rdi
	ret


#; increases the board at rsi by 1 into rdi
#; rdi -> [dstboard] base
#; rsi -> [srcboard] base
#; rdx -> number of items
applyrule:
	push rbp
	mov rbp, rsp
	sub rsp, 16
	#; ^ two local variables to keep track of src/dst board begin of row

	push rbx  #; size divided by 2 or 3
	push r12  #; outer loop index
	push r13  #; inner loop index
	push r14  #; -> srcboard, while in inner loop
	push r15  #; -> dstboard, while in inner loop

	mov [rbp-8], rsi
	mov [rbp-16], rdi
	test rdx, 1
	jnz ar_by3

ar_by2:
	mov rbx, rdx
	shr rbx
	mov r12, rbx
ar_by2outloop:
	mov r14, [rbp-8]
	mov r15, [rbp-16]
	mov r13, rbx
	ar_by2inloop:
		lea rdi, rules2[rip]
		mov rsi, r14
		call findmatch2
		#; rax <- base of the match, need +4 because skip 2x2 from rule itself
		#; row1
		mov dl, 4[rax]
		mov 0[r15], dl
		mov dl, 5[rax]
		mov 1[r15], dl
		mov dl, 6[rax]
		mov 2[r15], dl
		#; row2
		mov dl, 7[rax]
		mov N+0[r15], dl
		mov dl, 8[rax]
		mov N+1[r15], dl
		mov dl, 9[rax]
		mov N+2[r15], dl
		#; row3
		mov dl, 10[rax]
		mov 2*N+0[r15], dl
		mov dl, 11[rax]
		mov 2*N+1[r15], dl
		mov dl, 12[rax]
		mov 2*N+2[r15], dl
		#; row copied, next column
		add r14, 2
		add r15, 3
		dec r13
		jnz ar_by2inloop
	add qword ptr [rbp-8], N
	add qword ptr [rbp-16], N
	dec r12
	jnz ar_by2outloop
	jmp ar_done

ar_by3:
	mov rax, rdx
	xor rdx, rdx
	mov rcx, 3
	div rcx
	mov rbx, rax
	mov r12, rbx
ar_by3outloop:
	mov r14, [rbp-8]
	mov r15, [rbp-16]
	mov r13, rbx
	ar_by3inloop:
		lea rdi, rules3[rip]
		mov rsi, r14
		call findmatch3
		#; rax <- base of the match, need +9 because skip 3x3 from rule itself
		#; row1
		mov dl,  9[rax]
		mov 0[r15], dl
		mov dl, 10[rax]
		mov 1[r15], dl
		mov dl, 11[rax]
		mov 2[r15], dl
		mov dl, 12[rax]
		mov 3[r15], dl
		#; row2
		mov dl, 13[rax]
		mov N+0[r15], dl
		mov dl, 14[rax]
		mov N+1[r15], dl
		mov dl, 15[rax]
		mov N+2[r15], dl
		mov dl, 16[rax]
		mov N+3[r15], dl
		#; row3
		mov dl, 17[rax]
		mov 2*N+0[r15], dl
		mov dl, 18[rax]
		mov 2*N+1[r15], dl
		mov dl, 19[rax]
		mov 2*N+2[r15], dl
		mov dl, 20[rax]
		mov 2*N+3[r15], dl
		#; row4
		mov dl, 21[rax]
		mov 3*N+0[r15], dl
		mov dl, 22[rax]
		mov 3*N+1[r15], dl
		mov dl, 23[rax]
		mov 3*N+2[r15], dl
		mov dl, 24[rax]
		mov 3*N+3[r15], dl
		#; row copied, next column
		add r14, 3
		add r15, 4
		dec r13
		jnz ar_by3inloop
	add qword ptr [rbp-8], N
	add qword ptr [rbp-16], N
	dec r12
	jnz ar_by3outloop

ar_done:
	pop r15
	pop r14
	pop r13
	pop r12
	pop rbx
	leave
	ret


main:
	lea rdi, board1[rip]
	#; -> .#.
	mov byte ptr     0[rdi], 0
	mov byte ptr     1[rdi], 1
	mov byte ptr     2[rdi], 0
	#; -> ..#
	mov byte ptr   N+0[rdi], 0
	mov byte ptr   N+1[rdi], 0
	mov byte ptr   N+2[rdi], 1
	#; -> ###
	mov byte ptr 2*N+0[rdi], 1
	mov byte ptr 2*N+1[rdi], 1
	mov byte ptr 2*N+2[rdi], 1

	lea rdi, rules2[rip]
	lea rsi, rules3[rip]
	call readrules

	lea rsi, board1[rip]
	lea rdi, board2[rip]
	mov rdx, 3
	call applyrule

	xor rax, rax
	ret
