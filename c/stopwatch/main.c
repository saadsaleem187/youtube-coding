#include <stdio.h>
#include <time.h>

int main() {
    time_t start, end;
    double elapsed_time;

    printf("Press Enter key to start the stopwatch...\n");
    getchar();

    time(&start);

    printf("Stopwatch started\n");
    printf("Press Enter key to stop the stopwatch...\n");
    getchar();

    time(&end);

    elapsed_time = difftime(end, start);

    printf("Elapsed time: %.2f seconds\n", elapsed_time);

    return 0;
}
