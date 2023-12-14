// Thomas van Vliet
//
// As they're making the final adjustments, they discover that their calibration document (your puzzle input) 
// has been amended by a very young Elf who was apparently just excited to show off her art skills. 
// Consequently, the Elves are having trouble reading the values on the document.

// The newly-improved calibration document consists of lines of text; each line originally contained a 
// specific calibration value that the Elves now need to recover. On each line, the calibration value 
// can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.


#include <stdio.h>
#include <assert.h>

char *FILENAME = "day1_input.txt";
int FoundNumbers[1000];
char NUMBERS[10] = "0123456789";

int isNumer(char character){
    return (character >= '0' && character <= '9');
}


int firstNumerInString(char string[]){
    int size = sizeof(string) / sizeof(string[0]);
    for(int i = 0; i < size; i++){
        if (isNumer(string[i])){
            return string[i];
        }
    }
    return -1;
}

int main() {

    printf("The first number is: %c", firstNumerInString("fjdskhf3hijsdfi"));
    
    //FILE *fileP = fopen(FILENAME, "r");


 

//  assert(x==7);
    return 0;
}
