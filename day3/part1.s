.data
	fmt: .string "%d\n"
	readn: .string "%d"
	tempn: .long 0

.text
	.global main

main:
	lea tempn(%rip), %rsi
	lea readn(%rip), %rdi
	mov $0, %rax
	call scanf@PLT

	mov tempn(%rip), %rdx  # current number towards 0
	dec %rdx  # once number is 0 it means we walked N steps
	jz done
	mov $0, %rsi  # i, on board
	mov $0, %rdi  # j, on board
	mov $2, %rax  # n, dimensions

mainloop:
	# 1 right
	inc %rdi
	dec %rdx
	jz done

	# n-1 up
	mov %rax, %rcx
	dec %rcx
goingup:
	dec %rsi
	dec %rdx
	jz done
	loop goingup

	# n left
	mov %rax, %rcx
goingleft:
	dec %rdi
	dec %rdx
	jz done
	loop goingleft

	# n down
	mov %rax, %rcx
goingdown:
	inc %rsi
	dec %rdx
	jz done
	loop goingdown

	# n right
	mov %rax, %rcx
goingright:
	inc %rdi
	dec %rdx
	jz done
	loop goingright

	add $2, %rax  # radius is larger now
	jmp mainloop

done:
	cmp $0, %rsi
	jge noswapi
	neg %rsi
noswapi:
	cmp $0, %rdi
	jge noswapj
	neg %rdi
noswapj:

	add %rdi, %rsi
	lea fmt(%rip), %rdi
	mov $0, %rax
	call printf@PLT

	ret

