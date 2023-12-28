#include <stdio.h>

/*
  Modify the temperature conversion program to print a heading above the table.
*/

/* print Fahrenheit-Celsius table
   for fahr = 0, 20, ..., 300; floatin gpoint version
 */
main(){
  float fahr, celsius;
  int lower, upper, step;

  lower = 0;  /* lower limit of temparature */
  upper = 300;  /* upper limit */
  step = 20;  /* step size */

  printf("%3c %6c\n", 'F', 'C');
  
  fahr = lower;
  while (fahr <= upper) {
    celsius = (5.0/9.0) * (fahr-32.0);
    printf("%3.0f %6.1f\n", fahr, celsius);
    fahr = fahr + step;
  }
}
