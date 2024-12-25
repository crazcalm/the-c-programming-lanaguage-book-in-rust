#include <stdio.h>

/* print Celsius-Fahrenheit table
      for cels = 0, 20, ..., 300 */
main()
{
  int lower, upper, step;
  float fahr, celsius;
  
  lower = 0;  /* lower limit of temparature */
  upper = 300;  /* upper limit */
  step = 20;  /* step size */

  printf("%3c %6c\n", 'C', 'F');
  
  celsius = lower;
  while (celsius <= upper) {
    fahr = (celsius * (9.0/5.0)) + 32;
    printf("%3.0f %6.1f\n", celsius, fahr);
    celsius = celsius + step;
  }
}
