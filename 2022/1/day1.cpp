#include <iostream>
#include <fstream>
#include <string>

int main(int argc, char** argv) {
    //open input
    std::ifstream input("input.txt");
    
    if(!input.is_open()) {
        std::cout << "Could not open stream..." << std::endl;
        return -1;
    }

    int most_calories = 0;
    int sec_calories = 0;
    int third_calories = 0;

    int curr_calories = 0;
    std::string calories;
    while(!input.eof()) {
        std::getline(input, calories);

        //blank line means changing of elves
        if(calories == "") {
            if(curr_calories > most_calories) {
                third_calories = sec_calories;
                sec_calories = most_calories;
                most_calories = curr_calories;
            } else if(curr_calories > sec_calories) {
                third_calories = sec_calories;
                sec_calories = curr_calories;
            } else if(curr_calories > third_calories) {
                third_calories = curr_calories;
            }
            curr_calories = 0;
            continue;
        }
        
        curr_calories += stoi(calories);
    }

    std::cout << "Calorie Count:\n";
    std::cout << "\t1st: " << most_calories << std::endl;
    std::cout << "\t2nd: " << sec_calories << std::endl;
    std::cout << "\t3rd: " << third_calories << std::endl;
    std::cout <<"Total: " << most_calories + sec_calories + third_calories << std::endl;
    return 1;
}
