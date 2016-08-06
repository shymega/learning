#include <stdio.h>

int main(void) {
  int i, num, result = 0;

  scanf("%d", &num);

  for(i = 1; i <= num; i++) {
    result += i;
  }

  printf("%d\n", result);

  return 0;
}
