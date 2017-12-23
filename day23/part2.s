.intel_syntax noprefix

.data
	.set ISIZE, 10        #; instruction size
	.set NREG, 8          #; number of registers available
	.set MAXP, 100*ISIZE  #; max program length (in total size)

	regs: .zero NREG*8
	prog: .zero MAXP      #; program is 1b instruction|1b reg[|4b value]

	invalidop: .string "INVALID OPERATION AT POSITION %ld.\n"

	fmt: .string "%ld\n"
	readn: .string "%ld"
	tempn: .space 8

	#; instruction set
	.set OP_SET, 1
	.set OP_SUB, 2
	.set OP_MUL, 3
	.set OP_JNZ, 4
	.set OP_JMP, 5  #; jgz (<> 0) <reg|val>
	.set OP_NOP, 6  #; jgz (== 0) <reg|val>
	.set OP_OUTBOUNDS, 7

	#; set this flag on the operand if 2nd is a value
	.set FLAG_VAL, 64
	.set OP_MASK, 255-FLAG_VAL

.text
	.global main


#; reads a program from stdin
readprog:
	push rbx
	lea rbx, prog[rip]
rp_loop:
	call getchar@PLT
	cmp al, '\n'
	jle rp_done  #; may also be -1 for end
	cmp al, 's'
	je rp_sop
	cmp al, 'm'
	je rp_mop
	cmp al, 'j'
	je rp_jop
	jmp rp_invalid

rp_sop:
	#; may be set or sub
	call getchar@PLT
	cmp al, 'e'
	je rp_seop
	cmp al, 'u'
	je rp_suop
	jmp rp_invalid
	rp_seop:
		mov byte ptr [rbx], OP_SET
		call getchar@PLT  #; 't'
		call getchar@PLT  #; ' '
		jmp rp_saveregint
	rp_suop:
		mov byte ptr [rbx], OP_SUB
		call getchar@PLT  #; 'b'
		call getchar@PLT  #; ' '
		jmp rp_saveregint
rp_mop:
	mov byte ptr [rbx], OP_MUL
	call getchar@PLT  #; 'u'
	call getchar@PLT  #; 'l'
	call getchar@PLT  #; ' '
	jmp rp_saveregint
rp_jop:
	mov byte ptr [rbx], OP_JNZ
	call getchar@PLT  #; 'n'
	call getchar@PLT  #; 'z'
	call getchar@PLT  #; ' '
	#; jump is a bit special case as first operand can be a value
	#; if this is the case we need to determine whether it's jmp/nop
	call getchar@PLT
	cmp al, 'a'
	jge rp_dosaveregint
	cmp al, '0'  #; <= '0' means '0' or '-'
	jg rp_jmpop
rp_nopop:
	mov byte ptr [rbx], OP_NOP
	jmp rp_dosaveint
rp_jmpop:
	mov byte ptr [rbx], OP_JMP
	jmp rp_dosaveint

rp_saveregint:  #; reads reg, val, newline; rbx += 10
	call getchar@PLT  #; reg
rp_dosaveregint:  #; saves reg
	sub al, 'a'
	mov 1[rbx], al
rp_dosaveint:  #; reads reg|val newline
	lea rdi, readn[rip]
	lea rsi, tempn[rip]
	xor rax, rax
	call scanf@PLT
	test rax, rax
	jz rp_saveregreg  #; if scanf returns 0, wasn't int, so another reg
		#; reg int
		or byte ptr 0[rbx], FLAG_VAL  #; non-zero means read int -> flag
		mov rax, tempn[rip]
		mov 2[rbx], rax
		jmp rp_saveregintdone
	rp_saveregreg:
		#; reg reg
		call getchar@PLT
		sub al, 'a'
		mov 2[rbx], al
	rp_saveregintdone:
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


#; rdi -> memloc to the registers of this program
#; rsi -> memloc to the first instruction to execute
#;
#; the program exits as soon as a rcv instruction is found with no data
#; in rax, returns the number of instructions executed before halting
executeprogram:
	xor rcx, rcx  #; count executed mul instructions
ep_loop:
	movzx rax, byte ptr 0[rsi]  #; save op in al
	movzx r8,  byte ptr 1[rsi]  #; save dst in r8
	shl r8, 3  #; and remember they occupy 8 bytes :)
	test al, FLAG_VAL
	jz ep_loadreg  #; no flag val, then load a register
ep_loadval:
	mov r9, 2[rsi]      #; save src immediate value in r9
	jmp ep_loaddone
ep_loadreg:
	movzx rdx, byte ptr 2[rsi]
	mov r9, [rdi+rdx*8]  #; save src value from reg in r9
ep_loaddone:
	add rsi, ISIZE  #; next operation
	and al, OP_MASK  #; remove the flags
	cmp al, OP_OUTBOUNDS
	jge ep_halt
ep_validop:
	lea rdx, ep_jumptable[rip]  #; base of jump table doesn't change
	movsx rax, dword ptr [rdx+rax*4]  #; offset value
	add rax, rdx  #; add base to the offset value
	jmp rax  #; jump to it
	.section	.rodata
	.align 4
ep_jumptable:
	.long ep_halt - ep_jumptable
	.long ep_eset - ep_jumptable
	.long ep_esub - ep_jumptable
	.long ep_emul - ep_jumptable
	.long ep_ejnz - ep_jumptable
	.long ep_ejmp - ep_jumptable
	.long ep_loop - ep_jumptable  #; nop is just going back to the loop
	.text

ep_eset:
	mov [rdi+r8], r9
	jmp ep_loop
ep_esub:
	sub [rdi+r8], r9
	jmp ep_loop
ep_emul:
	mov rax, [rdi+r8]
	imul rax, r9
	mov [rdi+r8], rax
	inc rcx  #; another mul
	jmp ep_loop
ep_ejnz:
	test qword ptr [rdi+r8], 0xffffffffffffffff
	jz ep_loop  #; jump only if not equal to zero
ep_ejmp:
	imul r9, ISIZE
	sub rsi, ISIZE  #; back to current instruction first
	add rsi, r9  #; now jump
	jmp ep_loop
ep_halt:
	sub rsi, ISIZE  #; back one instruction, so we can re-execute it
ep_exit:
	mov rax, rcx  #; return in rax the number of executed instructions
	ret

main:
	#; dprintf ep_loop, " \t-> %ld\n", *($rdi+$r8)
	#; ignore 1 1
	#; dprintf ep_validop, "%c [= %ld] (%s) %ld", ($r8/8)+'a', *($rdi+$r8), ((char**)({"nop","set","sub","mul","jnz","jmp","nop"})[$al]), $r9
	push rbx
	push r12
	push r13
	call readprog

	lea rdi, regs[rip]
    mov qword ptr [rdi], 1  #; set a = 1
	lea rsi, prog[rip]
	call executeprogram

	lea rdi, fmt[rip]
	mov rsi, rax
	xor rax, rax
	call printf@PLT

	pop r13
	pop r12
	pop rbx
	xor rax, rax
	ret
