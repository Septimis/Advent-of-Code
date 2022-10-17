/* Number of failed Attempts: 1 */

#include <iostream>
#include <fstream>
#include <string>
#include <map>

std::map<std::string, int> getCoordinates(int a_index, std::string a_instruction) {
    std::map<std::string, int> l_coordinates;
    bool l_comma_encountered = false;
    std::string x1 = "", y1 = "", x2 = "", y2 = "";

    while(a_instruction[a_index] != ' ') {
        if(a_instruction[a_index] == ',') {
            l_comma_encountered = true;
            a_index++;
            continue;
        }
        if(!l_comma_encountered) x1 += a_instruction[a_index];
        else y1 += a_instruction[a_index];

        a_index++;
    }
    l_comma_encountered = false; //reset
    a_index += 9; //size of 'through '

    //get x2, y2
    while(a_index < a_instruction.length()) {
        if(a_instruction[a_index] == ',') {
            l_comma_encountered = true;
            a_index++;
            continue;
        }
        if(!l_comma_encountered) x2 += a_instruction[a_index];
        else y2 += a_instruction[a_index];

        a_index++;
    }

    l_coordinates["x1"] = std::stoi(x1);
    l_coordinates["y1"] = std::stoi(y1);
    l_coordinates["x2"] = std::stoi(x2);
    l_coordinates["y2"] = std::stoi(y2);

    return l_coordinates;
}

int main() {
    std::ifstream input("input.txt");
    
    //initialize the light grid
    int** light_grid = new int*[1000];
    for(int i = 0; i < 1000; i++) {
        light_grid[i] = new int[1000];

        for(int j = 0; j < 1000; j++)
            light_grid[i][j] = 0;
    }

    std::string instruction;
    while(!input.eof()) {
        std::getline(input, instruction);
        if(instruction == "") continue; //in case of a blank line

        std::map<std::string, int> coordinates;

        if(instruction.substr(0, 8) == "turn off") {
            coordinates = getCoordinates(9, instruction);

            for(int i = coordinates["x1"]; i <= coordinates["x2"]; i++)
                for(int j = coordinates["y1"]; j <= coordinates["y2"]; j++)
                    if(light_grid[i][j] > 0) light_grid[i][j]--;
            
        } else if(instruction.substr(0, 7) == "turn on") {
            coordinates = getCoordinates(8, instruction);

            for(int i = coordinates["x1"]; i <= coordinates["x2"]; i++)
                for(int j = coordinates["y1"]; j <= coordinates["y2"]; j++)
                    light_grid[i][j]++;

        } else if(instruction.substr(0,6) == "toggle") {
            coordinates = getCoordinates(7, instruction);

            for(int i = coordinates["x1"]; i <= coordinates["x2"]; i++)
                for(int j = coordinates["y1"]; j <= coordinates["y2"]; j++)
                    light_grid[i][j] += 2;

        } else {
            std::cout << "unknown instruction '" << instruction << "'.  Exiting..." << std::endl;
            return 0;
        }
    }

    //check how many lights are lit
    int brightness = 0;
    for(int i = 0; i < 1000; i++)
        for(int j = 0; j < 1000; j++)
            brightness += light_grid[i][j];
    
    std::cout << "Number of lights lit: " << brightness << std::endl;

    //deallocate memory
    for(int i = 0; i < 1000; i++)
        delete[] light_grid[i];
    delete[] light_grid;

    return 1;
}