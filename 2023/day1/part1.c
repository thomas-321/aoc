// Thomas van Vliet
//
// day 1 part 1
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) 
// has been amended by a very young Elf who was apparently just excited to show off her art skills. 
// Consequently, the Elves are having trouble reading the values on the document.

// The newly-improved calibration document consists of lines of text; each line originally contained a 
// specific calibration value that the Elves now need to recover. On each line, the calibration value 
// can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.


#include <stdio.h>
#include <string.h>

char *FILENAME = "input.txt";

int isNumer(char character){
    return (character >= '0' && character <= '9');
}


char firstNumerInString(char *string, size_t size){
    for(int i = 0; i < size; i++){
        if (isNumer(string[i])){
            return string[i];
        }
    }
    return '0';
}


char lastNumerInString(char *string, size_t size){
    for(int i = size - 1; i >= 0; i--){
        if (isNumer(string[i])){
            return string[i];
        }
    }
    return '0';
}

int combineNumberCharacter(char *a, char *b){
    int x = (*a) - '0';
    int y = (*b) - '0';
    return x * 10 + y *1;

}

int main() {

    int number = 0;
    const unsigned MAX_LENGTH = 256;
    char buffer[MAX_LENGTH];

    FILE *fp = fopen(FILENAME, "r");
    while (fgets(buffer, MAX_LENGTH, fp)){
        size_t lineLength = strlen(buffer);
        char a = firstNumerInString(&buffer[0], lineLength);
        char b = lastNumerInString(&buffer[0], lineLength);
        int combindNumber = combineNumberCharacter(&a,&b);
        number += combindNumber;
    }
    fclose(fp);

    printf("The calibration value = %i", number);

    return 0;
}
