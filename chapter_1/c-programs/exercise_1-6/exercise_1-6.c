#include <stdio.h>

main()
{
  int c, count;

  count = 0;
  while((c = getchar()) != EOF) {
    if (c == '\n') {
      ++count;
    }
    if (c == ' ') {
      ++count;
    }
    if (c == '\t'){
      ++count;
    }
  }
  
  printf("%d\n", count);
}
