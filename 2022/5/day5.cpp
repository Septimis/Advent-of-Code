#include <iostream>
#include <fstream>
#include <string>
#include <deque>

int main() {
    std::ifstream input("input.txt");
    if(!input.is_open()) {
        std::cout << "Could not open input file.  Quitting...";
        return -1;
    }

    std::string line;
    std::deque<char> crane_stacks;

    //create deque
    std::getline(input, line);

    
    //loop to read in the crane input
    while(!input.eof()) {
        std::getline(input, line);
        if(line == "") break;

        for(int i = 0; i < line.length(); i++) {
            if(i % 4 == 1 && line[i] >= 65 && line[i] <= 90) {
            }
        }
    }

    //build data structures to represent data


    //loop to go through instructions

    return 0;
}
