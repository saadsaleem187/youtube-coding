#include <stdio.h>

int main() {
    int c, spaces;

    spaces = 0;

    while ((c = getchar()) != EOF) {
        if (c == ' ') {
            ++spaces;
        }
    }

    printf("%d\n", spaces);

    return 0;
}
