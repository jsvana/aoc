#include <array>
#include <fstream>
#include <iostream>
#include <tuple>

enum class ParserState {
  NORMAL,
  GARBAGE,
};

const int parse(const std::string& line) {
  std::size_t i = 0;
  auto state = ParserState::NORMAL;
  int garbage = 0;

  while (i < line.length()) {
    const auto c = line[i];

    switch (state) {
      case ParserState::NORMAL:
        switch (c) {
          case '<':
            state = ParserState::GARBAGE;
            break;
        }
        break;
      case ParserState::GARBAGE:
        if (c == '>') {
          state = ParserState::NORMAL;
        } else if (c == '!') {
          ++i;
        } else {
          ++garbage;
        }
        break;
    }

    ++i;
  }

  return garbage;
}

int main(int argc, char** argv) {
  std::ifstream input_f(argv[1]);
  std::string line;
  std::getline(input_f, line);

  std::cout << parse(line) << std::endl;
}
