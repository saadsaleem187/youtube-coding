#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>

void display_clock();
void clear_screen();

int main() {
    display_clock();

    return 0;
}

void display_clock() {
    while (1) {
        time_t raw_time;
        struct tm *time_info;

        time(&raw_time);
        time_info = localtime(&raw_time);

        clear_screen();

        printf("Digital Clock\n");
        printf("----------------\n");
        printf(" Time: %02d:%02d:%02d\n",
                time_info->tm_hour,
                time_info->tm_min,
                time_info->tm_sec);
        printf("----------------\n");

        sleep(1);
    }
}

void clear_screen() {
    #ifdef _WIN32
        system("cls");
    #else
        system("clear");
    #endif
}
