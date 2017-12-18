.intel_syntax noprefix

.data
	.set NREG, 32       #; number of registers
	.set MAXP, 2048     #; max program length
	regs: .zero NREG*8  #; NREG 8 bytes-long registers
	prog: .zero MAXP    #; program is 1 byte instruction|1 byte reg[|4 value]

	invalidop: .string "INVALID OPERATION AT POSITION %ld.\n"

	fmt: .string "%ld\n"
	readn: .string "%ld"
	tempn: .space 8

	#; instruction set
	#; 0: (halt program)
	#; 1: snd <reg>
	#; 2: set <reg> <val>
	#; 3: add <reg> <val>
	#; 4: mul <reg> <val>
	#; 5: mod <reg> <val>
	#; 6: rcv <reg>
	#; 7: jgz <reg> <val>
	.set OP_SND, 1
	.set OP_SET, 2
	.set OP_ADD, 3
	.set OP_MUL, 4
	.set OP_MOD, 5
	.set OP_RCV, 6
	.set OP_JGZ, 7
	.set OP_OUTBOUNDS, 8

.text
	.global main


#; reads a program from stdin
readprog:
	push rbx
	lea rbx, prog[rip]
rp_loop:
	call getchar@PLT
	cmp al, '\n'
	je rp_done
	cmp al, 's'
	je rp_sop
	cmp al, 'a'
	je rp_aop
	cmp al, 'm'
	je rp_mop
	cmp al, 'r'
	je rp_rop
	cmp al, 'j'
	je rp_jop
	jmp rp_invalid

rp_sop:
	#; may be snd or set
	call getchar@PLT
	cmp al, 'n'
	je rp_snop
	cmp al, 'e'
	je rp_seop
	jmp rp_invalid
	rp_snop:
		mov byte ptr [rbx], OP_SND
		call getchar@PLT  #; 'd'
		call getchar@PLT  #; ' '
		jmp rp_savereg
	rp_seop:
		mov byte ptr [rbx], OP_SET
		call getchar@PLT  #; 't'
		call getchar@PLT  #; ' '
		jmp rp_saveregint
rp_aop:
	mov byte ptr [rbx], OP_ADD
	call getchar@PLT  #; 'd'
	call getchar@PLT  #; 'd'
	call getchar@PLT  #; ' '
	jmp rp_saveregint
rp_mop:
	#; may be mul or mod
	call getchar@PLT
	cmp al, 'u'
	je rp_muop
	cmp al, 'o'
	je rp_moop
	jmp rp_invalid
	rp_muop:
		mov byte ptr [rbx], OP_MUL
		call getchar@PLT  #; 'l'
		call getchar@PLT  #; ' '
		jmp rp_saveregint
	rp_moop:
		mov byte ptr [rbx], OP_MOD
		call getchar@PLT  #; 'd'
		call getchar@PLT  #; ' '
		jmp rp_saveregint
rp_rop:
	mov byte ptr [rbx], OP_RCV
	call getchar@PLT  #; 'c'
	call getchar@PLT  #; 'v'
	call getchar@PLT  #; ' '
	jmp rp_savereg
rp_jop:
	mov byte ptr [rbx], OP_JGZ
	call getchar@PLT  #; 'g'
	call getchar@PLT  #; 'z'
	call getchar@PLT  #; ' '
	jmp rp_saveregint

rp_savereg:  #; reads reg, newline; rbx += 2
	call getchar@PLT  #; reg
	sub al, 'a'
	mov 1[rbx], al
	add rbx, 2
	call getchar@PLT
	jmp rp_loop
rp_saveregint:  #; reads reg, val, newline; rbx += 10
	call getchar@PLT  #; reg
	sub al, 'a'
	mov 1[rbx], al
	lea rdi, readn[rip]
	lea rsi, tempn[rip]
	xor rax, rax
	call scanf@PLT
	mov rax, tempn[rip]
	mov 2[rbx], rax
	add rbx, 10
	call getchar@PLT
	jmp rp_loop
rp_invalid:
	lea rdi, invalidop[rip]
	mov rsi, rbx
	lea rax, prog[rip]
	sub rsi, rax
	xor rax, rax
	call printf@PLT
rp_done:
	pop rbx
	ret


#; executes the program in [prog]
#; returns in rax the latest sound played
executeprogram:
	#; reminder: 'lodsb' is equivalent to 'al = [rsi++]'
	lea rsi, prog[rip]
	lea rdi, regs[rip]
	xor r8, r8    #; latest sound sent
	xor rcx, rcx  #; temporary data
ep_loop:
	xor rax, rax  #; read values
	lodsb
	cmp al, OP_OUTBOUNDS
	jge ep_halt
	lea rdx, ep_jumptable[rip]  #; base of jump table
	movsx rax, dword ptr [rdx+rax*4]  #; offset value
	add rax, rdx  #; add base to the offset value
	jmp rax  #; jump to it
	.section	.rodata
	.align 4
ep_jumptable:
	.long ep_halt - ep_jumptable
	.long ep_esnd - ep_jumptable
	.long ep_eset - ep_jumptable
	.long ep_eadd - ep_jumptable
	.long ep_emul - ep_jumptable
	.long ep_emod - ep_jumptable
	.long ep_ercv - ep_jumptable
	.long ep_ejgz - ep_jumptable
	.text

ep_esnd:
	lodsb
	mov r8, [rdi+rax]  #; send sound
	jmp ep_loop
ep_eset:
	lodsb
	mov cl, al
	lodsq
	mov [rdi+rcx], rax
	jmp ep_loop
ep_eadd:
	lodsb
	mov cl, al
	lodsq
	add [rdi+rcx], rax
	jmp ep_loop
ep_emul:
	lodsb
	mov cl, al
	lodsq
	mov rdx, [rdi+rcx]
	imul rdx, rax
	mov [rdi+rcx], rdx
	jmp ep_loop
ep_emod:
	lodsb
	mov cl, al
	lodsq
	xor rdx, rdx
	idiv qword ptr [rdi+rcx]
	mov [rdi+rcx], rdx
	jmp ep_loop
ep_ercv:
	lodsb
	cmp qword ptr [rdi+rax], 0
	jne ep_halt  #; recover last sound if reg <> 0
	jmp ep_loop
ep_ejgz:
	lodsb
	mov cl, al
	lodsq
	cmp qword ptr [rdi+rcx], 0
	jle ep_loop  #; jump only if greater than zero
	#; TODO jump.. with variable length encoding.. ouch
	jmp ep_loop
ep_halt:
	mov rax, r8
	ret


main:
	call readprog
	call executeprogram

	lea rdi, fmt[rip]
	mov rsi, rax
	xor rax, rax
	call printf@PLT

	xor rax, rax
	ret
