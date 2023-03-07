#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <regex>
#include <algorithm>
#include <map>

enum Choice {
    Rock = 1,
    Paper = 2,
    Sissors = 3,
};

enum Result {
    Lost = 0,
    Draw = 3,
    Win = 6,
};

// shamelessly stolen from stackoverflow because c++ can't split strings natively...
std::vector<std::string> split(const std::string str, const std::string regex_str)
{
    std::regex regexz(regex_str);
    std::vector<std::string> list(std::sregex_token_iterator(str.begin(), str.end(), regexz, -1),
                                  std::sregex_token_iterator());
    return list;
}

int main(int argc, char* argv[])
{
    auto fname = "guide.txt";
    std::map<std::string, const Choice> TheirChoiceMap = {
        {"A", Choice::Rock},
        {"B", Choice::Paper},
        {"C", Choice::Sissors},
    };
    std::map<std::string, const Choice> MyChoiceMap = {
        {"X", Choice::Rock},
        {"Y", Choice::Paper},
        {"Z", Choice::Sissors},
    };
    // Open the file
    std::ifstream f;
    f.open(fname, std::ios_base::in);
    if (!f.is_open()) {
        std::cerr << "Failed to open file " << fname << "\n";
        return 1;
    }
    // Parse the file
    auto line = std::string();
    auto my_score = 0ull;
    while (f.good()) {
        std::getline(f, line);
        line = std::regex_replace(line, std::regex("\\r\\n|\\r|\\n"), "");
        auto round_choice = split(line, " ");
        if (round_choice.empty()) {
            std::cerr << "Failed to parse file " << fname << "\n";
            return 1;
        }
        const auto their_choice = TheirChoiceMap[round_choice[0]];
        const auto my_choice = MyChoiceMap[round_choice[1]];

        Result round_result;
        if (my_choice == Choice::Rock && their_choice == Choice::Rock) {
            round_result = Result::Draw;
        } else if (my_choice == Choice::Rock && their_choice == Choice::Paper) {
            round_result = Result::Lost;
        } else if (my_choice == Choice::Rock && their_choice == Choice::Sissors) {
            round_result = Result::Win;
        } else if (my_choice == Choice::Paper && their_choice == Choice::Rock) {
            round_result = Result::Win;
        } else if (my_choice == Choice::Paper && their_choice == Choice::Paper) {
            round_result = Result::Draw;
        } else if (my_choice == Choice::Paper && their_choice == Choice::Sissors) {
            round_result = Result::Lost;
        } else if (my_choice == Choice::Sissors && their_choice == Choice::Rock) {
            round_result = Result::Lost;
        } else if (my_choice == Choice::Sissors && their_choice == Choice::Paper) {
            round_result = Result::Win;
        } else if (my_choice == Choice::Sissors && their_choice == Choice::Sissors) {
            round_result = Result::Draw;
        } else {
            std::cerr << "Failed to parse round result from " << fname << "\n";
            return 1;
        }

        my_score += (unsigned long long)round_result;
        my_score += (unsigned long long)my_choice;
    }
    std::cout << "Total Score: " << my_score << "\n";
    return 0;
}