#include <stdio.h>
#include <string.h>

const char *FILENAME = "input.txt";
const unsigned int MAX_LINE_LENGTH = 256;

const char *NUMBERS[19] = {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

int numbersIndexToString (int index) {
    return index < 10 ? index : index - 10 + 1;
}


int firstNumberInString(char *string, size_t size){
    int indexFirstNumber = MAX_LINE_LENGTH;
    int firstNumber;
    int indexInArray;
    char* pArrayIndex;

    for(int i = 0; i < 19; i++){
        pArrayIndex = strstr(string, NUMBERS[i]); 
        
        // if number not in string -> skip
        if (pArrayIndex == NULL) {
            continue;            
        }

        indexInArray = pArrayIndex - string;

        // if index is smaller then current index
        if (indexInArray < indexFirstNumber) {
            indexFirstNumber = indexInArray;
            firstNumber = i;
        }
    }

    return numbersIndexToString(firstNumber);
}


int lastNumberInString(char *string, size_t size){
    int indexLastNumber = -1;
    int lastNumber = 0;
    int indexInArray;
    char* pArrayIndex;

    for(int i = 0; i < 19; i++){
        pArrayIndex = strstr(string, NUMBERS[i]); 
       
        // while loop to keep looking after first match in case of exmp. onefourone
        // needed because strstr loops from start to end 
        while (pArrayIndex != NULL) {
            indexInArray = pArrayIndex - string;
            
            if (indexInArray > indexLastNumber) {
                indexLastNumber = indexInArray;
                lastNumber = i;
            }
            
            pArrayIndex = strstr(pArrayIndex + strlen(NUMBERS[i]), NUMBERS[i]); 
        }
    }
    return numbersIndexToString(lastNumber); 
}


int numberOnLine (char* buffer, int bufLength) { 
    int fNm = firstNumberInString(buffer, bufLength);
    int lNm = lastNumberInString(buffer, bufLength);
    return fNm * 10 + lNm;
}


int main () {
    int number = 0;
    char buffer[MAX_LINE_LENGTH];
    FILE *fp = fopen(FILENAME, "r");
    
    while (fgets(buffer, MAX_LINE_LENGTH, fp)){
         number += numberOnLine(buffer, strlen(buffer)); 
    }
    
    fclose(fp);

    printf("The calibration value = %i", number);
    // answer 54431
}
