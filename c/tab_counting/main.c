#include <stdio.h>

int main() {
    int c, nt;

    nt = 0;

    while ((c = getchar()) != EOF) {
        if (c == '\t') {
            ++nt;
        }
    }

    printf("%d\n", nt);

    return 0;
}
