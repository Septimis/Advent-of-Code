#include <iostream>
#include <fstream>

int main() {
	int floorNum = 0;
	std::ifstream input("input.txt");

	char direction;
	int index = 0;
	int basement_index = -1;
	while(input.get(direction)) {
		if(direction == '(')
			floorNum++;
		else if(direction == ')')
			floorNum--;

		if(floorNum == -1 && basement_index == -1)
			basement_index = index + 1; //add 1 to offset the zero index

		index++;
	}
	std::cout << "ending floor: " << floorNum  << "\nbasement entered at index " << basement_index << std::endl;
	input.close();

	return 1;
}
