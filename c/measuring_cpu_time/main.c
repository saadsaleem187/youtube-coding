#include <stdio.h>
#include <time.h>

int main() {
    clock_t start, end;
    double cpu_time;

    start = clock();
    
    for (long i = 0; i < 1000000000; i++);

    end = clock();

    cpu_time = (double)(end - start) / CLOCKS_PER_SEC;

    printf("CPU time used: %.2f seconds.\n", cpu_time);

    return 0;
}
