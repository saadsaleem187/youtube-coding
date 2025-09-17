#include <stdio.h>

int main() {
    int c;

    printf("Enter text (Ctrl+D to end on Linux/Mac or Ctrl+Z on Windows):\n");

    while ((c = getchar()) != EOF) {
        if (c == ' ' || c == '\t' || c == '\n') {
            printf("\n");
        } else {
            putchar(c);
        }
    }

    return 0;
}
