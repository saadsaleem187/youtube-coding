#include <stdio.h>

void writeToFile(const char *filename);
void readFromFile(const char *filename);

int main() {
    const char *filename = "numbers.txt";
    
    writeToFile(filename);

    readFromFile(filename);

    return 0;
}

void writeToFile(const char *filename) {
    FILE *file = fopen(filename, "w");

    if (file == NULL) {
        printf("Error opening file for writing.\n");
        return;
    }

    for (int i = 1; i <= 5; i++) {
        fprintf(file, "%d\n", i);
    }

    printf("\nData written to file successfully.\n");

    fclose(file);
}

void readFromFile(const char *filename) {
    FILE *file = fopen(filename, "r");

    if (file == NULL) {
        printf("Error opening file for reading.\n");
        return;
    }

    printf("\nReading data from file.\n\n");
    
    int number;

    while (fscanf(file, "%d", &number) != EOF) {
        printf("%d\n", number);
    }

    fclose(file);
}
