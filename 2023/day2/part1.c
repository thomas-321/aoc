#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char *FILENAME = "test.txt";
const unsigned int MAX_LINE_LENGTH = 256;

const int redCubeAmount = 12;
const int greenCubeAmount = 13;
const int blueCubeAmount = 14;



int getId(char* string, size_t sLength) {
    char* temp;
    char* stopSearch;
    char* searchChar = ":";
    char number[3];
    int totalNumbersFound = 0;

    stopSearch = strstr(searchChar, string);

    temp = string;
    while(temp != stopSearch){
        if (isdigit(temp)) {
            number[totalNumbersFound] = *temp;
            totalNumbersFound++;
        }
    }

    number[totalNumbersFound] = '\0';
    return atoi(number);
}

int largestRedAmount(char* string, size_t sLength){

    return 1;
}

int idIfThrowPossible(char* string, size_t sLength) {
    if (!largestRedAmount(string, sLength)){
        return 0;
    }

    return getId(string, sLength);   
}

int main () {
    int number = 0;
    char buffer[MAX_LINE_LENGTH];
    FILE *fp = fopen(FILENAME, "r");
    
    while (fgets(buffer, MAX_LINE_LENGTH, fp)){
        printf("ID: %i", getId(buffer, MAX_LINE_LENGTH));
    }
    
    fclose(fp);

    

   // printf("The calibration value = %i", number);
    // answer 54431
}

