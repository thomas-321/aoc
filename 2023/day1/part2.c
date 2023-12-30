// Thomas van Vliet
//
// day 1 part 1
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) 
// has been amended by a very young Elf who was apparently just excited to show off her art skills. 
// Consequently, the Elves are having trouble reading the values on the document.

// The newly-improved calibration document consists of lines of text; each line originally contained a 
// specific calibration value that the Elves now need to recover. On each line, the calibration value 
// can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.


#include <limits.h>
#include <stdio.h>
#include <string.h>

char *FILENAME = "input.txt";
char *NUMBERS[19] = {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

int isNumer(char character){
    return (character >= '0' && character <= '9');
}



char numberStringToChar(int character){
    char newChar;
    if ( character >= 0 && character <= 9 ){
        return '0' + character;
    }
    return '0';
}



char firstNumerInString(char *string){
    int indexFirstNumber = 0;
    int index = INT_MAX;
    int temp = INT_MAX;

    for(int i = 0; i < 19; i++){
        if( strstr(string, NUMBERS[i]) ){ 
            temp = strstr(string, NUMBERS[i])-string;
            if ( temp < index ){
                indexFirstNumber = i;
                index = temp;
            }
        }
    }
    if( index == INT_MAX ) { return '0'; }
    if ( index > 10 ){       return numberStringToChar(indexFirstNumber - 10); } 
    return numberStringToChar(indexFirstNumber);
}


char lastNumerInString(char *string, size_t size){
    int indexFirstNumber = 0;
    int index = INT_MAX;
    int temp = 0;

    for(int i = 0; i < 19; i++){
        if( strstr(string, NUMBERS[i]) ){ 
            temp = strstr(string, NUMBERS[i])-string;
            if ( temp > index ){
                indexFirstNumber = i;
                index = temp;
            }
        }
    }
    if( index == INT_MAX ) { return '0'; }
    if ( index > 10 ){       return numberStringToChar(indexFirstNumber - 10); } 
    return numberStringToChar(indexFirstNumber);


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
        char a = firstNumerInString(&buffer[0]);
        char b = lastNumerInString(&buffer[0], lineLength);
        printf("a: %c b: %c \n", a, b);
        //int combindNumber = combineNumberCharacter(&a,&b);
        number += combineNumberCharacter(&a, &b);
    }
    fclose(fp);

    printf("The calibration value = %i", number);

    return 0;
}
