#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#ifdef _WIN32
    #define CLEAR_COMMAND "cls"
#else
    #define CLEAR_COMMAND "clear"
#endif

int main() {
    int seconds;

    printf("Enter countdown time in seconds: ");
    scanf("%d", &seconds);

    while (seconds >= 0) {
        system(CLEAR_COMMAND);
        printf("Countdown: %d\n", seconds);
        sleep(1);
        seconds--;
    }
    
    printf("Time's Up!\n");

    return 0;
}
