#include <iostream>
#include <string>
#include <fstream>

int main(int argc, char** argv) {
    std::ifstream input("input.txt");
    if(!input.is_open()) {
        std::cout << "Cannot open file..." << std::endl;
    }

    int part1_score = 0;
    int part2_score = 0;
    std::string round;
    while(!input.eof()) {
        std::getline(input, round);
        if(round == "") continue; // skip blank lines

        char opponent_move = round[0]; //A for rock, B for paper, C for scissors
        char my_move = round[2]; //X for rock, Y for paper, Z for scissors
        char outcome = round[2]; //X for loss, Y for draw, Z for win

        //award points for what shape I selected
        //part1
        switch(my_move) {
            case 'X':
                part1_score += 1;
                break;
            case 'Y':
                part1_score += 2;
                break;
            case 'Z':
                part1_score += 3;
                break;
            default:
                std::cout << "Unknown move '" << my_move << "'..." << std::endl;
                break;
        }
        //part2
        if( //rock
                (outcome == 'X' && opponent_move == 'B') ||
                (outcome == 'Y' && opponent_move == 'A') ||
                (outcome == 'Z' && opponent_move == 'C')
          ) part2_score += 1;
        if( //paper
                (outcome == 'X' && opponent_move == 'C') || 
                (outcome == 'Y' && opponent_move == 'B') ||
                (outcome == 'Z' && opponent_move == 'A')
          ) part2_score += 2;
        if( //scissors
                (outcome == 'X' && opponent_move == 'A') ||
                (outcome == 'Y' && opponent_move == 'C') ||
                (outcome == 'Z' && opponent_move == 'B')
          ) part2_score += 3;

        //award points for the outcome of the round
        //part1
        if( //win
                (my_move == 'X' && opponent_move == 'C') ||
                (my_move == 'Y' && opponent_move == 'A') ||
                (my_move == 'Z' && opponent_move == 'B')
          ) part1_score += 6;
        if( //draw
                (my_move == 'X' && opponent_move == 'A') ||
                (my_move == 'Y' && opponent_move == 'B') ||
                (my_move == 'Z' && opponent_move == 'C')
          ) part1_score += 3;
        //part2
        switch(outcome) {
            case 'Y':
                part2_score += 3;
                break;
            case 'Z':
                part2_score += 6;
                break;
        }
    }

    std::cout << "Part 1 Score: " << part1_score << std::endl;
    std::cout << "Part 2 Score: " << part2_score << std::endl;

    return 0;
}
