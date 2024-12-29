#include <stdio.h>

main()  /* copy input to oupt; 1st version */
{
  int c;

  c = getchar();
  while (c != EOF) {
    putchar(c);
    c = getchar();
  }
}
