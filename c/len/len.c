#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>

int
main (int argc, char **argv) {
    char *str;
    unsigned int len;

    if (argc < 2) {
        fprintf (stderr, "ERROR: Not enough arguments passed.\n");
        return EXIT_FAILURE;
    }

    str = argv[1];
    len = (unsigned int) strlen (str);

    if (len <= 0) {
        fprintf (stderr, "ERROR: String length is less than OR equal to 0.\n");
        return EXIT_FAILURE;
    }

    fprintf (stdout, "%d\n",
            len);

    return EXIT_SUCCESS;
}
