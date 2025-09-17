#include <stdio.h>

void checkBalance(float balance);
float deposit(float balance);
float withdraw(float balance);
void exitATM();

int main() {
    int choice;
    float balance = 1000.00;

    printf("====================================\n");
    printf("        Welcome to Mini ATM \n" );
    printf("====================================\n");
    
    while (1) {
        printf("\nPlease select an option:\n\n");
        printf("1. Check Balance\n");
        printf("2. Deposit Money\n");
        printf("3. Withdraw Money\n");
        printf("4. Exit\n\n");
        printf("Your choice: ");
        scanf("%d", &choice);

        switch (choice) {
            case 1:
                checkBalance(balance);
                break;
            case 2:
                balance = deposit(balance);
                break;
            case 3:
                balance = withdraw(balance);
                break;
            case 4:
                exitATM();
                return 0;
            default:
                printf("Invalid choice. Please try again.\n");
        }
    }

    return 0;
}

void checkBalance(float balance) {
    printf("\nYour current balance is: $%.2f\n", balance);
}

float deposit(float balance) {
    float amount;

    printf("\nEnter the amount to deposit: $");
    scanf("%f", &amount);

    if (amount > 0) {
        balance += amount;
        printf("You have successfully deposited $%.2f\n", amount);
        printf("Your new balance is: $%.2f\n", balance);
    } else {
        printf("Invalid deposit amount. Please try again.\n");
    }

    return balance;
}

float withdraw(float balance) {
    float amount;

    printf("Enter the amount to withdraw: $");
    scanf("%f", &amount);

    if (amount > 0 && amount <= balance) {
        balance -= amount;
        printf("You have successfully withdrawn $%.2f\n", amount);
        printf("Your new balance is: $%.2f\n", balance);
    } else if (amount > balance) {
        printf("Insufficient funds! Your balance is: $%.2f\n", balance);
    } else {
        printf("Invalid withdraw amount. Please try again.\n");
    }

    return balance;
}

void exitATM() {
    printf("\nThank you for using Mini ATM. Have a great day.\n\n");
}
