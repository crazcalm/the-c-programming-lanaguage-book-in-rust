#include <stdio.h>

#define YES 1
#define NO  0

main()  /* count lines, words, chars in input */
{
  int c, inword;

  inword = NO;
  while ((c = getchar()) != EOF){
    if (c == ' ' || c == '\n' || c == '\t'){
      if (inword == YES) {
	printf("\n");
      }
      inword = NO;
    }
    else if (inword == NO) {
      inword = YES;
      printf("%c", c);
    }
    else {
      printf("%c", c);
    }
  }
}
