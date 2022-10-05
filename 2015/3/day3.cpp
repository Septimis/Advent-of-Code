#include <iostream>
#include <fstream>
#include <vector>

class Coordinates {
    public:
        int x;
        int y;
        int num_times_visited;

        Coordinates(int a_x, int a_y) {
            x = a_x;
            y = a_y;
            num_times_visited = 1;
        }
};

bool isVisited(int a_x, int a_y, std::vector<Coordinates> &a_houses_visided) {
    for(int i = 0; i < a_houses_visided.size(); i++) {
        if(a_x == a_houses_visided[i].x && a_y == a_houses_visided[i].y) {
            a_houses_visided[i].num_times_visited++;
            return true;
        }
    }
    a_houses_visided.push_back(Coordinates(a_x, a_y));
    return false;
}

int main() {
    std::ifstream input("input.txt");
    std::vector<Coordinates> houses_visited;
    bool robo_turn = false;
    int x = 0;
    int y = 0;
    int robo_x = 0;
    int robo_y = 0;
    houses_visited.push_back(Coordinates(x, y)); // push initial position

    char direction;
    while(input.get(direction)) {
        switch (direction) {
            case '^':
                robo_turn ? robo_y++ : y++;
                break;
            case '>':
                robo_turn ? robo_x++ : x++;
                break;
            case 'v':
                robo_turn ? robo_y-- : y--;
                break;
            case '<':
                robo_turn ? robo_x-- : x--;
                break;
            default:
                std::cout << "Invalid input '" << direction << "', exiting..." << std::endl;
                return 0;
                break;
        }

        robo_turn? isVisited(robo_x, robo_y, houses_visited) : isVisited(x, y, houses_visited);
        robo_turn = !robo_turn;
    }
    std::cout << "Number of houses visited at least once: " << houses_visited.size() << std::endl;

    input.close();
    return 1;
}