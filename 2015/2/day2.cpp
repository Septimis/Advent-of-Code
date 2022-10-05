#include <iostream>
#include <fstream>
#include <string>

int main() {
	std::ifstream input("input.txt");
	int sqft_wrapping_paper = 0;
    int ft_ribbon = 0;

	if(!input.is_open()) {
		std::cout << "Error opening input stream" << std::endl;
		return 0;
	}
	std::string dimensions;
	while(!input.eof()) {
		std::getline(input, dimensions);
		int length = 0,
            width = 0,
            height = 0;
		
		std::string dimension = "";
		int dimension_index = 0;
		for(int i = 0; i < dimensions.length(); i++) {
			if(dimensions[i] != 'x')
				dimension += dimensions[i];
			else {
				switch(dimension_index) {
					case 0:
						length = stoi(dimension);
						break;
					case 1:
						width = stoi(dimension);
						break;
				}
				dimension = "";

				dimension_index++;
			}
		}
        height = stoi(dimension);
        dimension = "";

		//calculate the different sides
		int side1 = length * width;
		int side2 = width * height;
		int side3 = height * length;

		//now test for smallest dimension for slack
		int smallest_dimension = -INT16_MAX;
		if(side1 <= side2 && side1 <= side3)
			smallest_dimension = side1;
		else if(side2 <= side1 && side2 <= side3)
			smallest_dimension = side2;
		else if(side3 <= side1 && side3 <= side2)
			smallest_dimension = side3;

		
		sqft_wrapping_paper += side1 * 2 + side2 * 2 + side3 * 2 + smallest_dimension;

        //calculate ribbon amount needed
        int ribbon_bow = length * width * height;
        int ribbon_wrap = -INT16_MAX;
        if(length <= width && length <= height) {
            if(width <= height)
                ribbon_wrap = length * 2 + width * 2;
            else
                ribbon_wrap = length * 2 + height * 2;
        } else if(width <= height && width <= length) {
            if(height <= length)
                ribbon_wrap = width *2 + height * 2;
            else
                ribbon_wrap = width * 2 + length * 2;
        } else if(height <= length && height <= width) {
            if(length <= width)
                ribbon_wrap = height * 2 + length * 2;
            else
                ribbon_wrap = height * 2 + width * 2;
        }
        
        ft_ribbon += ribbon_wrap + ribbon_bow;

	}

	std::cout << "wrapping paper: " << sqft_wrapping_paper << std::endl;
    std::cout << "ribbon: " << ft_ribbon << std::endl;
	input.close();

	return 1;
}
