#include <iostream>
#include <fstream>
#include <string>

int getPriority(int badge) {
    if(badge >= 97 && badge <= 122) //lowercase
        return badge - 96;
    else if(badge >= 65 && badge <= 90) //uppercase
        return badge - 64 + 26;
    return -1;
}

int main(int argc, char** argv) {
    std::ifstream input("input.txt");
    if(!input.is_open()) {
        std::cout << "Can't read input.txt" << std::endl;
        return -1;
    }

    int priority = 0;
    int group_priority = 0;
    int index = 0;
    std::string sack1;
    std::string sack2;
    std::string rucksack;
    while(!input.eof()) {
        std::getline(input, rucksack);
        if(rucksack == "") continue;  //skip blank lines

        bool sack_match_found = false;
        if(index % 3 == 0)
            sack1 = rucksack;
        else if(index % 3 == 1)
            sack2 = rucksack;
        else {
            //find the commonality between the three rucksacks
            for(int i = 0; i < sack1.length() && !sack_match_found; i++)
                for(int j = 0; j < sack2.length() && !sack_match_found; j++)
                    for(int k = 0; k < rucksack.length() && !sack_match_found; k++)
                        if(sack1[i] == sack2[j] && sack1[i] == rucksack[k]) {
                            group_priority += getPriority((int)sack1[i]);
                            sack_match_found = true;
                        }
        }

        //get each compartment
        std::string compartment_1 = rucksack.substr(0, rucksack.length() / 2);
        std::string compartment_2 = rucksack.substr(rucksack.length() / 2, rucksack.length() - 1);

        bool found_match = false;
        //loop through and find the matching pair
        for(int i = 0; i < compartment_1.length(); i++) {
            for(int j = 0; j < compartment_2.length(); j++) {
                if(compartment_1[i] == compartment_2[j]) { //check for match
                    priority += getPriority(compartment_1[i]);
                    found_match = true;
                    break;
                }
            }
            if(found_match) break;
        }
        index++;
    }

    std::cout << "Part 1 Priority: " << priority << std::endl;
    std::cout << "Part 2 Priority: " << group_priority << std::endl; 

    return 0;
}
