#include <iostream>
#include <fstream>
#include <string>

int main() {
    // if(testStr.find("dd") != std::string::npos) std::cout << "position: " << testStr.find("dd") << std::endl;
    std::ifstream input("input.txt");
    int num_nice_string = 0;

    std::string str_on_trial;
    while(!input.eof()) {
        std::getline(input, str_on_trial);
        bool rule1 = false;
        bool rule2 = false;

        for(int i = 0; i < str_on_trial.length(); i++) {
            if(i < str_on_trial.length() - 2 && str_on_trial[i] == str_on_trial[i + 2]) rule2 = true;

            for(int j = 0; j < str_on_trial.length(); j++) {
                if(j == i || j == i + 1 || j + 1 == i) continue; //overlapping rule
                if(
                    j < str_on_trial.length() - 1 &&
                    i < str_on_trial.length() - 1 &&
                    str_on_trial.substr(i, 2) == str_on_trial.substr(j, 2)
                ) rule1 = true;
            }
        }

        if(rule1 && rule2)
            num_nice_string++;
    }

    std::cout << "Number of nice strings: " << num_nice_string << std::endl;

    return 1;
}