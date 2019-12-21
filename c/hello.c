#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef enum {
    Mon = 0,
    Tue = 1,
    Wed = 2,
    Thu = 3,
    Fri = 4,
    Sat = 5,
    Sun = 6,
} Week;

typedef struct {
    char *Name;
    char Gender[1];
    unsigned int Age;
} Person;

void say_hello(char *name) {
    printf("Hello, %s!\n", name);
}

int main(void) {
    Person person;
    person.Name = "Dom";
    person.Age = 21;
    person.Gender = ['M'];

    fprintf(stdout, "Hello, %s\n",
            person.Name);

    return EXIT_SUCCESS;
}
