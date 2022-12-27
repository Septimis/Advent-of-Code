#include <iostream>
#include <fstream>
#include <string>

//726 is too low

int main(int argc, char** argv) {
    std::ifstream input("input.txt");
    if(!input.is_open()) {
        std::cout << "Cannot open input.txt" << std::endl;
        return -1;
    }

    int number_contained_pairs = 0;
    int number_overlapped_pairs = 0;
    std::string pair;
    while(!input.eof()) {
        std::getline(input, pair);
        if(pair == "") continue; //skip blank lines

        //split into lower/upper bounds of each range
        int range1_lower,
            range1_upper,
            range2_lower,
            range2_upper;
        
        std::string buffer = "";
        int number_counter = 0;
        for(int i = 0; i < pair.length(); i++) {
            if(!((int)pair[i] >= 48 && (int)pair[i] <= 57)) { //Is not a number
                switch(number_counter) {
                    case 0:
                        range1_lower = std::stoi(buffer);
                        break;
                    case 1:
                        range1_upper = std::stoi(buffer);
                        break;
                    case 2:
                        range2_lower = std::stoi(buffer);
                        break;
                    default:
                        std::cout << "Unknown number counter: " << number_counter << std::endl;
                        break;
                }
                buffer = "";
                number_counter++;
            } else buffer = buffer + pair[i];
        }
        range2_upper = std::stoi(buffer);

        //check if the ranges are contained
        if(
            (range1_lower <= range2_lower && range1_upper >= range2_upper) ||
            (range2_lower <= range1_lower && range2_upper >= range1_upper)
          ) number_contained_pairs++;

        //check if the ranges overlap
        if(!(range1_upper < range2_lower || range1_lower > range2_upper))
            number_overlapped_pairs++;
    }

    std::cout << "Number of contained pairs: " << number_contained_pairs << std::endl;
    std::cout << "Number of overlapped pairs: " << number_overlapped_pairs << std::endl;

    return 0;
}
