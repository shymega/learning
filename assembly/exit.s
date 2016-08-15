	# Setup the DATA section
	.section .data
	# Setup the TEXT section
	.section .text
	# Make the _start section global.
	.globl _start

# Define the _start section.
_start:

	# Move the integer '1' into the %eax register
	# This prepares for the Linux system call
	movl $1, %eax

	# Move the integer '4' into the %ebx register
	# This is the exit code for the program.
	movl $4, %ebx

	#Interrupt the program, and give Linux the control.
	int $0x80
