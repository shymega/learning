#include <stdio.h>
#include <stdlib.h>

#include <editline/readline.h>
#include <histedit.h>

#include "mpc.h"

int main(int argc, char *argv[]) {
  puts("SHYL Version 0.0.1");
  puts("Hit C-c to exit\n");

  while (1) {

    char* input = readline("SHYL> ");

    add_history(input);

    printf("> No, you're a %s", input);

    free(input);
  }

  return 0;
}
