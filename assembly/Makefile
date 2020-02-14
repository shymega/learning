.POSIX:
.SILENT:


clean:
    rm -v exit hello-world

main: clean exit hello-world

exit:
		nasm -f elf64 -o exit.o exit.asm
		ld exit.o -o exit
		rm exit.o

hello-world:
		nasm -f elf64 -o hello-world.o hello-world.asm
		ld -o hello-world hello-world.o
		rm hello-world.o

.PHONY: main exit hello-world clean;
