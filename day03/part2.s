.data
	fmt: .string "%d\n"
	readn: .string "%d"
	tempn: .long 0

	.set N, 10  # matrix dimensions
	matrix: .zero N*N*8

.text
	.global main

# (%rsi, %rdi) = (i, j) in the matrix assuming it's 0
# uses %rax, %r11, %rdx
# %rax contains the sum
sumneigh:
	mov $0, %rdx
	mov %rsi, %rax
	mov $N, %r11
	mul %r11
	add %rdi, %rax

	lea matrix(%rip), %r11  # base access
	mov %rax, %rdx  # indexed access
	mov $0, %rax  # accumulator

	# start from top left corner
	sub $N, %rdx
	dec %rdx
	add (%r11, %rdx, 8), %rax
	inc %rdx
	add (%r11, %rdx, 8), %rax
	inc %rdx
	add (%r11, %rdx, 8), %rax

	add $N, %rdx
	sub $2, %rdx
	add (%r11, %rdx, 8), %rax
	inc %rdx
	add (%r11, %rdx, 8), %rax
	inc %rdx
	add (%r11, %rdx, 8), %rax

	add $N, %rdx
	sub $2, %rdx
	add (%r11, %rdx, 8), %rax
	inc %rdx
	add (%r11, %rdx, 8), %rax
	inc %rdx
	add (%r11, %rdx, 8), %rax

	# back to the center
	sub $N, %rdx
	dec %rdx
	mov %rax, (%r11, %rdx, 8)
	ret

main:
	lea tempn(%rip), %rsi
	lea readn(%rip), %rdi
	mov $0, %rax
	call scanf@PLT

	mov tempn(%rip), %r8  # target number
	mov $N/2, %rsi  # i, on board
	mov $N/2, %rdi  # j, on board

	# set initial number to 1
	mov $0, %rdx
	mov %rsi, %rax
	mov $N, %r11
	mul %r11
	add %rdi, %rax
	lea matrix(%rip), %rcx
	movq $1, (%rcx, %rax, 8)

	# down the rabbit hole we go
	mov $2, %r9  # n
mainloop:
	# 1 right
	inc %rdi
	call sumneigh
	cmp %r8, %rax
	jg done

	# n-1 up
	mov %r9, %rcx
	dec %rcx
goingup:
	dec %rsi
	call sumneigh
	cmp %r8, %rax
	jg done
	loop goingup

	# n left
	mov %r9, %rcx
goingleft:
	dec %rdi
	call sumneigh
	cmp %r8, %rax
	jg done
	loop goingleft

	# n down
	mov %r9, %rcx
goingdown:
	inc %rsi
	call sumneigh
	cmp %r8, %rax
	jg done
	loop goingdown

	# n right
	mov %r9, %rcx
goingright:
	inc %rdi
	call sumneigh
	cmp %r8, %rax
	jg done
	loop goingright

	add $2, %r9  # radius is larger now
	jmp mainloop

done:
	mov %rax, %rsi
	lea fmt(%rip), %rdi
	mov $0, %rax
	call printf@PLT

	mov $0, %rax
	ret

