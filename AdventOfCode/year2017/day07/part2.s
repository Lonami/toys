.intel_syntax noprefix

.data
	#; two arrays, one will hold names, the other weights
	#; thanksfully, names are 7 chars or less, so they fit
	#; in a single register!
	.set N, 2048  #; maximum number of lines, not actual lines
	names: .zero N*8  #; program names
	weights: .zero N*8  #; program weights
	toweights: .zero N*8  #; tower weights

	#; now to store who holds who, we have a matrix
	#; [[who, holds...]], N rows of max M-1 holds of 8 size
	.set HOLDERS, 16
	holders: .zero 512*HOLDERS*8

	#; scanf can read the '(weight)' for us just fine
	weightfmt: .string "(%ld)"
	tempn: .quad

.text
	.global main

#; reads a name and puts it in rax
#; rbx holds the last character read that caused the exit
readname:
	xor rbx, rbx
readnameloop:
	call getchar@PLT
	cmp al, ' '
	je readnamedone
	cmp al, '\n'
	je readnamedone
	cmp al, ','
	je readnamedone

	shl rbx, 8
	mov bl, al
	jmp readnameloop
readnamedone:
	xchg rax, rbx
	ret


#; shows a name on screen, must be in rax
showname:
	push rax
	push rbx
	mov rbx, rax

shownameloop:
	rol rbx, 8
	test bl, bl
	jz shownamenext
	movzbq rdi, bl
	xor bl, bl
	call putchar@PLT

shownamenext:
	test rbx, rbx
	jnz shownameloop
	mov rdi, '\n'
	call putchar@PLT

	pop rbx
	pop rax
	ret


#; gets the index from name to -> [names]
#; r12 contains the base of the names
#; rax contains the name, uses rcx
#; rdi will contain index
getindex:
	mov rdi, r12
	mov rcx, N
	repne scasq
	sub rdi, r12
	sub rdi, 8  #; we overshoot by one
	ret


#; gets the index from name -> [holders]
#; r14 contains the base of the holders
#; rdx contains the name
#; rax will contain zero if not found, and rsi the index
getholderindex:
	mov rsi, -HOLDERS*8
getholderindexloop:
	add rsi, HOLDERS*8
	mov rax, [r14+rsi]
	test rax, rax
	jz getholderindexdone
	cmp rax, rdx
	jne getholderindexloop
getholderindexdone:
	ret


#; r12 contains the base of the names
#; r13 contains the base of the weights
#; r14 contains the base of the holders
#; r15 contains the base of the tower weights
#; rdi contains the index of program name/weight/tower weight
calctowers:
	#; for the given tower, we need to find who holds
	#; we recursively call this method to add all the
	#; weights up, and return that
	push rbx
	push rsi
	push rdi

	mov rdx, [r12+rdi]  #; rdx holds the name
	mov rbx, [r13+rdi]  #; rbx holds the sum

	#; find the list of holders, rsi contains row index
	call getholderindex
	test rax, rax
	jz calctowers_done

	#; we found this in the list of who holds!
calctowers_sumloop:
	add rsi, 8
	mov rax, [r14+rsi]
	test rax, rax  #; empty name, we're done with holders
	jz calctowers_done

	#; find the index of this name (in rax name, out rdi index)
	call getindex
	call calctowers
	add rbx, rax  #; result is in rax, add to our accum
	jmp calctowers_sumloop

calctowers_done:
	mov rax, rbx

	pop rdi
	mov [r15+rdi], rax

	pop rsi
	pop rbx
	ret


#; r14 contains the base of the holders
#; r15 contains the base of the tower weights
#; rsi contains the index (to start of row) of holders
#;
#; uses rcx (when calling getindex)
#; returns in rax 1 if it's unbalanced or 0 if balanced
isunbal:
	push rbx
	push rdx  #; hold from memory and on division

	mov rbx, 0  #; counter of held towers
	mov rdx, 0  #; weight accumulator

isunballoop:
	add rsi, 8
	mov rax, [r14+rsi]
	test rax, rax
	jz isunballdone

	#; find the index of this name and then its weight
	call getindex
	mov rax, [r15+rdi]

	add rdx, rax
	inc rbx
	jmp isunballoop

isunballdone:
	#; we have the total weight and how many towers.
	#; we divide this weight by the count of towers.
	#; if the result weight is not equal to any (e.g.
	#; the last tower being held), then it means that
	#; it's unbalanced, because there's no way for for
	#; the weight to be equal assuming only one is wrong
	sub rsi, 8  #; back to last tower

	#; now we perform division.. somehow
	xchg rax, rdx  #; rax is zero, zero out rdx, save acc in rax
	div rbx

	cmp rax, [r14+rsi]
	setne al  #; not equal, unbalanced, then al = 1
	and rax, 0xff

	pop rdx
	pop rbx
	ret


#; finds the highest depth index of which tower is unbalanced.
#; this algorithm works by keeping track of both the best depth
#; so far and its associated index, updating it when better are
#; found (bigger depth, also unbalanced). returns said index.
#;
#; bases: r12 of names, r13 of weights, r14 of holders, r15 of tower weights
#; rdi contains the current tower index
#; r8 contains bigger depth, r9 the index
findunbal:
	mov rdx, [r12+rdi]
	call getholderindex

	test rax, rax
	jz findunbal_done

findunbal_whichloop:
	add rsi, 8


findunbal_done:
	ret
 note: i don't feel like doing this, i'm tired


main:
	push rbp
	push r12
	push r13
	push r14
	push r15

	lea r12, names[rip]
	lea r13, weights[rip]
	lea r14, holders[rip]
readloop:
	call readname
	test rax, rax
	jz readdone  #; simply a new line (empty name) means done

	mov rbp, rax  #; save original name in rbp for later
	mov [r12], rax
	lea rdi, weightfmt[rip]
	lea rsi, tempn[rip]
	mov rax, 0
	call scanf@PLT
	mov rax, tempn[rip]
	mov [r13], rax

	call getchar@PLT
	cmp al, '\n'
	je readnext
	#; else, we read a space, then comes '-> '
	call getchar@PLT
	call getchar@PLT
	call getchar@PLT

	#; original name in rbp, so we know the holder
	#; r15 will contain the size of the added holders
	mov [r14], rbp
	add r14, 8
	mov r15, 8
readholders:
	call readname

	#; always save the name
	mov [r14], rax
	add r14, 8
	add r15, 8

	#; check if we read a newline, then we're done
	cmp bl, '\n'
	je readholdersdone

	#; else bl = ',' (read the space so to consume the whole ', ')
	call getchar@PLT
	jmp readholders

readholdersdone:
	#; next row of holders
	sub r14, r15
	add r14, HOLDERS*8

readnext:
	add r12, 8
	add r13, 8
	jmp readloop
readdone:

	#; now that we've finally read all data,
	#; we need to find who's at the bottom. but who?
	#; well, simply the one who's not held by anyone!
	lea r12, names[rip]

	#; r12 will be the iterator over names
	#; r14 will be the iterator of ROWS over holders
findbottom_loop:
	mov rax, [r12]  #; with this name, search in holders
	add r12, 8
	lea r14, holders[rip]  #; start again for every name

findbottom_rowloop:
	mov rcx, [r14]
	add r14, HOLDERS*8
	test rcx, rcx
	jz found_bottom  #; no holders in clean row, end!

	mov rbx, 0  #; index inside current holders row
findbottom_holderloop:
	inc rbx
	mov rcx, [r14+rbx*8]
	test rcx, rcx
	jz findbottom_rowloop  #; no more holders here, next row

	cmp rcx, rax
	je findbottom_loop  #; we found someone holding us
	jmp findbottom_holderloop  #; check more holders

found_bottom:
	call showname

	#; reload the indices for them to be correct
	lea r12, names[rip]
	lea r13, weights[rip]
	lea r14, holders[rip]
	lea r15, toweights[rip]

	#; the name is still in the rax
	call getindex
	#; now the index is in rdi, calculate all the weights
	call calctowers

	#; the weights of the towers has been calculated
	#; the index on rdi hasn't been lost. now we need
	#; to find where it's unbalanced

	pop r15
	pop r14
	pop r13
	pop r12
	pop rbp
	xor rax, rax
	ret
