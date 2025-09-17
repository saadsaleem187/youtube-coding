#include <stdio.h>

#define LOWER 0
#define UPPER 300
#define STEP 20

int main() {
    float fahr, celsius;
    
    fahr = LOWER;
    
    printf("Fahrenheit\tCelsius\n");

    while (fahr <= UPPER) {
        celsius = (5.0/9.0) * (fahr - 32.0);
        printf("%6.0f%16.1f\n", fahr, celsius);
        fahr = fahr + STEP;
    }
}
