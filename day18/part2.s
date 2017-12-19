.intel_syntax noprefix

.data
	.set ISIZE, 10        #; instruction size
	.set NREG, 32         #; number of registers available
	.set MAXP, 100*ISIZE  #; max program length (in total size)

	regs0: .zero NREG*8
	regs1: .zero NREG*8   #; NREG 8 bytes-long registers
	prog: .zero MAXP      #; program is 1b instruction|1b reg[|4b value]

	#; queues consist of (offset to 1st val, offset to end of queue, queue)
	.set QUEUE_SIZE, 10000
	queue0: .zero 8+8+QUEUE_SIZE*8
	queue1: .zero 8+8+QUEUE_SIZE*8

	invalidop: .string "INVALID OPERATION AT POSITION %ld.\n"

	fmt: .string "%ld\n"
	readn: .string "%ld"
	tempn: .space 8

	#; instruction set
	#; 0: (halt program)
	#; 1: snd <reg>
	#; 2: set <reg> <reg|val>
	#; 3: add <reg> <reg|val>
	#; 4: mul <reg> <reg|val>
	#; 5: mod <reg> <reg|val>
	#; 6: rcv <reg>
	#; 7: jgz <reg> <reg|val>
	.set OP_SND, 1
	.set OP_SET, 2
	.set OP_ADD, 3
	.set OP_MUL, 4
	.set OP_MOD, 5
	.set OP_RCV, 6
	.set OP_JGZ, 7
	.set OP_OUTBOUNDS, 8

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
	add rbx, 10
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
#; rdx -> memloc to the base queue for THIS program
#; rcx -> memloc to the base queue of the TARGET program
#;
#; the program exits as soon as a rcv instruction is found with no data
#; in rax, returns the number of instructions executed before halting
executeprogram:
	mov r10, rdx
	mov r11, rcx
	xor rcx, rcx  #; count executed instructions
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
	inc rcx  #; another valid instruction
	lea rdx, ep_jumptable[rip]  #; base of jump table doesn't change
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
	mov rdx, [rdi+r8]  #; load the data to send
	mov rax, 8[r11]  #; determine the index where we can insert
	mov 16[r11+rax*8], rdx
	inc rax  #; next available slot is one farther now
	xor rdx, rdx  #; need a register for cmov
	cmp rax, QUEUE_SIZE
	cmove rax, rdx  #; wrap around the queue size -> count not reliable though
	mov 8[r11], rax
	jmp ep_exit
	#;jmp ep_loop
ep_eset:
	mov [rdi+r8], r9
	jmp ep_loop
ep_eadd:
	add [rdi+r8], r9
	jmp ep_loop
ep_emul:
	mov rax, [rdi+r8]
	imul rax, r9
	mov [rdi+r8], rax
	jmp ep_loop
ep_emod:
	mov rax, [rdi+r8]
	xor rdx, rdx
	idiv r9
	mov [rdi+r8], rdx
	jmp ep_loop
ep_ercv:
	mov rax, 0[r10]
	mov rdx, 8[r10]
	cmp rax, rdx
	je ep_halt  #; start and end are the same, no data in the queue
	mov rdx, 16[r10+rax*8]  #; load data from queue
	mov [rdi+r8], rdx  #; into our register
	inc rax  #; next slot we need to pick up is one farther
	xor rdx, rdx  #; need a register for cmov
	cmp rax, QUEUE_SIZE
	cmove rax, rdx  #; wrap around the queue size though!
	mov 0[r10], rax
	jmp ep_loop
ep_ejgz:
	cmp qword ptr [rdi+r8], 0
	jle ep_loop  #; jump only if greater than zero
	imul r9, ISIZE
	sub rsi, ISIZE  #; back to current instruction first
	add rsi, r9  #; now jump
	jmp ep_loop
ep_halt:
	sub rsi, ISIZE  #; back one instruction, so we can re-execute it
	dec rcx
ep_exit:
	mov rax, rcx  #; return in rax the number of executed instructions
	ret
#;dprintf 218, "%ld\t>> [%ld]\n", $rdx, $rax
#;dprintf 247, "%ld\t<< [%ld]\n", $rdx, $rax
main:
	push rbx
	push r12
	push r13
	call readprog

	#; program 0 has a 0 in their 'p' register, and program 1 has a 1
	mov qword ptr regs0+('p'-'a')*8[rip], 0
	mov qword ptr regs1+('p'-'a')*8[rip], 1

	#; save the current instruction index of each program on r12/r13
	#; so that they can resume where they left it on the next execution
	lea r12, prog[rip]
	mov r13, r12

	#; TODO uhh figure out how to detect deadlocks :D
	#; dprintf callrecvloop, "queue0(start = %ld, end = %ld)\n", 
callrecvloop:
	xor rbx, rbx  #; save how many instructions were ran by both
	lea rdi, regs0[rip]
	mov rsi, r12
	lea rdx, queue0[rip]
	lea rcx, queue1[rip]
	call executeprogram
	mov r12, rsi
	add rbx, rax

	lea rdi, regs1[rip]
	mov rsi, r13
	lea rdx, queue1[rip]
	lea rcx, queue0[rip]
	call executeprogram
	mov r13, rsi
	add rbx, rax

	jnz callrecvloop  #; exit as soon as they run zero instructions

	#; how many times program 1 sent a value is determined by
	#; the end position of program's 0 queue-1 so that's nice
	lea rdi, fmt[rip]
	lea rax, queue0[rip]
	mov rsi, 8[rax]
	dec rsi
	xor rax, rax
	call printf@PLT

	pop r13
	pop r12
	pop rbx
	xor rax, rax
	ret
