#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <regex>
#include <algorithm>

int main(int argc, char* argv[])
{
    auto fname = "calories.txt";
    std::ifstream f;
    f.open(fname, std::ios_base::in);
    if (!f.is_open()) {
        std::cerr << "Failed to open file " << fname << "\n";
        return 1;
    }
    
    auto line = std::string();
    auto all_calories = std::vector<uint64_t>();
    uint64_t total_calories = 0;
    while (f.good()) {
        std::getline(f, line);
        line = std::regex_replace(line, std::regex("\\r\\n|\\r|\\n"), "");
        if (line.empty()) {
            all_calories.push_back(total_calories);
            total_calories = 0;
            continue;
        }
        total_calories += std::stoull(line);
    }
    auto iter = std::max_element(all_calories.begin(), all_calories.end());
    if (iter == all_calories.end()) {
        std::cerr << "Invalid iterator!\n";
        return 1;
    }
    std::cout << "Elf with the highest calorie count is: " << *iter << "!\n";
    return 0;
}
