#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#define NUMERIC "0123456789"
#define LOWERCASE "abcdefghijklmnopqrstuvwxyz"
#define UPPERCASE "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
#define SPECIAL "!@#$%^&*()_-+=<>?/|{}[]~"

void generate_password(int length);

int main() {
    int length;

    printf("Welcome to the Password Generator.\n");
    printf("Enter the desired password length: ");
    scanf("%d", &length);

    if (length <= 0) {
        printf("Invalid password length.\n");
        return 1;
    }
    
    generate_password(length);

    return 0;
}

void generate_password(int length) {
    char all_chars[] = NUMERIC LOWERCASE UPPERCASE SPECIAL;
    char char_set_size = sizeof(all_chars) - 1;

    srand(time(0));

    printf("Generated Password: ");

    for (int i = 0; i < length; i++) {
        printf("%c", all_chars[rand() % char_set_size]);
    }

    printf("\n");
}












