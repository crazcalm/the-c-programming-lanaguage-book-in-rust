#include <stdio.h>

#define IN_WHITESPACE  1
#define OUT_WHITESPACE 0


main()
{
  int c, state;

  state = OUT_WHITESPACE;

  while((c = getchar()) != EOF) {
    if (c != ' ') {
      putchar(c);

     if (state == IN_WHITESPACE) {
       state = OUT_WHITESPACE;
      }
    }

    if (c == ' ') {
      if (state == OUT_WHITESPACE){
        putchar(c);

        state = IN_WHITESPACE;
      }
    }
    
  }
}
