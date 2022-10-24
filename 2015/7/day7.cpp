#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <vector>

//bitwise AND -> &
//bitwise OR -> |
//bitwise NOT -> !
//bitwise XOR -> ^
//bitwise SHIFT-R -> >>
//bitwise SHIFT-L -> <<

std::vector<std::string> getCommands(std::string command) {
    std::vector<std::string> commands;

    std::string tmp_cmd = "";

    for(char c : command) {
        if(c == ' ')
            commands.push_back(tmp_cmd);
        else
            tmp_cmd = tmp_cmd + c;
    }

    return commands;
}

int main(int argc, char** argv) {
    std::ifstream input("input.txt");

    std::map<std::string, unsigned short int> wires;

    std::string instruction;
    while(!input.eof()) {
        std::getline(input, instruction);
        if(instruction == "") continue; //skip blank lines

        std::vector<std::string> commands = getCommands(instruction);
    }

    return 1;
}
