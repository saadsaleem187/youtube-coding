#include <stdio.h>

int main() {
    int age;
    float balance;
    char grade;
    char name[50];

    printf("Enter your age: ");
    scanf("%d", &age);
    
    printf("Enter your account balance: ");
    scanf("%f", &balance);
    
    printf("Enter your grade (A, B, C...): ");
    scanf(" %c", &grade);
    
    printf("Enter your name: ");
    scanf(" %49[^\n]", name);

    printf("\n=== User Information ===\n");
    printf("Name: %s\n", name);
    printf("Age: %d\n", age);
    printf("Grade: %c\n", grade);
    printf("Account Balance: %.2f\n", balance);

    return 0;
}
