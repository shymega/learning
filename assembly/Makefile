exit:
		nasm -f elf64 -o exit.o exit.asm
		ld exit.o -o exit
		rm exit.o

maximum:
		as maximum.s -o maximum.o
		ld maximum.o -o maximum
		rm maximum.o

hello-world:
		nasm -f elf64 -o hello-world.o hello-world.asm
		ld -o hello-world hello-world.o
		rm hello-world.o
