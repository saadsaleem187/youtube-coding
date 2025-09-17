#include <stdio.h>

#define IN 1 /* inside a word */
#define OUT 0 /* outside a word */

int main() {
    int c, nw, state;

    nw = 0;
    state = OUT;

    while((c = getchar()) != EOF) {
        if (c == ' ' || c == '\n' || c == '\t') {
            state = OUT; 
        } else if (state == OUT) {
            state = IN;
            ++nw;
        }
    }

    printf("%d\n", nw);

    return 0;
}
