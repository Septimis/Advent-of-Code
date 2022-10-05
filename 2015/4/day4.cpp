#include <iostream>
#include <string>
#include "md5.hpp"

bool hasXZeros(int a_num_zeros, std::string a_hash) {
    if(a_hash.length() > a_num_zeros) {
        for(int i = 0; i < a_num_zeros; i++) {
            if(a_hash[i] != '0') return false;
        }
    } else return false;

    return true;
}

int main() {
    std::string input = "ckczppom";

    int index = 0;
    while(!hasXZeros(5, md5(input + std::to_string(index))))
        index++;

    std::cout << "index appended to get 5 leading 0's: " << index << std::endl;

    index = 0;
    while(!hasXZeros(6, md5(input + std::to_string(index))))
        index++;

    std::cout << "index appended to get 6 leading 0's: " << index << std::endl;
    return 1;
}