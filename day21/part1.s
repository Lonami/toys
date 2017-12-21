.intel_syntax noprefix

.data
	.set MAXR2, 8
	.set MAXR3, 128
	.set N, 1024  #; board dimensions

	fmt: .string "%d\n"

	#; 0 means off, 1 means on, -1 means not set
	rules2: .space MAXR2*13, -1  #; 2*2 + 3*3 = 4 +  9
	rules3: .space MAXR3*25, -1  #; 3*3 + 4*4 = 9 + 16

	board: .zero N*N

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
#; rax <- index of the matching rule, or -1
findmatch2:
	xor rax, rax  #; index of the matching rule
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
	inc rax
	add rdi, 25
	cmp byte ptr [rdi], -1
	jne fm2_loop
fm2_finalfail:
	mov rax, -1
fm2_done:
	ret


#; rdi -> [rules3] base
#; rsi -> [top left corner] base
#; rax <- index of the matching rule, or -1
findmatch3:
	xor r11, r11  #; index of the matching rule
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
	inc r11
	add rdi, 25
	cmp byte ptr [rdi], -1
	jne fm3_loop
fm3_finalfail:
	mov r11, -1
fm3_done:
	mov rax, r11
	ret

main:
	lea rdi, board[rip]
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

	lea rdi, rules3[rip]
	lea rsi, board[rip]
	call findmatch3

	lea rdi, fmt[rip]
	mov rsi, rax
	xor rax, rax
	call printf@PLT

	xor rax, rax
	ret
