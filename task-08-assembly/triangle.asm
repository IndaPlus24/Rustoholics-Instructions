##
# Push value to application stack.
# PARAM: Registry with value.
##
.macro	PUSH (%reg)
	addi	sp,sp,-4              # decrement stack pointer (stack builds "downwards" in memory)
	sw	%reg,0(sp)             # save value to stack
.end_macro

##
# Pop value from application stack.
# PARAM: Registry which to save value to.
##
.macro	POP (%reg)
	lw	%reg,0(sp)             # load value from stack to given registry
	addi	sp,sp,4               # increment stack pointer (stack builds "downwards" in memory)
.end_macro

.data
S:     .asciz "*"     # define label HW as a constant string "Hello World\n"
NL:    .asciz "\n"     # define label HW as a constant string "Hello World\n"


.text

main:
	li a0, 10
	j loop

loop:
	ble a0, zero, exit_program
	
	add	s0, a0, zero            # save return address
	
	jal print
	
	li      a7, 4                  # set syscall code "print string"
    la      a0, NL                 # load address of string HW into syscall argument registry
    ecall                          # print "Hello World\n" to standard output stream
	
    add	a0, s0, zero            # restore return adress
    
    addi a0, a0, -1
    
    j loop
	

print:
	add	s1, a0, zero            # save return address

	li      a7, 4                  # set syscall code "print string"
    la      a0, S                  # load address of string HW into syscall argument registry
    ecall                          # print "Hello World\n" to standard output stream
	
	add	a0, s1, zero
	
	addi a0, a0, -1
	
	bgt a0, zero, print
	jr ra


exit_program:
    # exit program
    li	a7, 10                      # set system call code to "terminate program"
    ecall 
