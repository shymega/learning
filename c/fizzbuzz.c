#include <stdio.h>
#include <stdlib.h>

int main(void) {
    for (int i = 1; i <= 100; i++) {
        if (i % 15 == 0) {
            fprintf(stdout, "FizzBuzz\n");
        } else if (i % 3 == 0) {
            fprintf(stdout, "Fizz\n");
        } else if (i % 5 == 0) {
            fprintf(stdout, "Buzz\n");
        } else {
            fprintf(stdout, "%d\n", i);
        }
    }

    return EXIT_SUCCESS;
}
