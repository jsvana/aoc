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
  int score = 0;
  int bracket_count = 0;

  while (i < line.length()) {
    const auto c = line[i];

    switch (state) {
      case ParserState::NORMAL:
        switch (c) {
          case '<':
            state = ParserState::GARBAGE;
            break;
          case '{':
            ++bracket_count;
            break;
          case '}':
            score += bracket_count;
            --bracket_count;
            break;
        }
        break;
      case ParserState::GARBAGE:
        if (c == '>') {
          state = ParserState::NORMAL;
        } else if (c == '!') {
          ++i;
        }
        break;
    }

    ++i;
  }

  if (state != ParserState::NORMAL) {
    std::cout << "WRONG STATE" << std::endl;
  }

  if (bracket_count != 0) {
    std::cout << "DOUBLE WAT: " << bracket_count << std::endl;
  }

  return score;
}

void test() {
  std::array<std::tuple<std::string, int>, 9> tests = {
      std::make_tuple("{}", 1),
      std::make_tuple("{{{}}}", 6),
      std::make_tuple("{{},{}}", 5),
      std::make_tuple("{{{},{},{{}}}}", 16),
      std::make_tuple("{<a>,<a>,<a>,<a>}", 1),
      std::make_tuple("{{<ab>},{<ab>},{<ab>},{<ab>}}", 9),
      std::make_tuple("{{<!!>},{<!!>},{<!!>},{<!!>}}", 9),
      std::make_tuple("{{<a!>},{<a!>},{<a!>},{<ab>}}", 3),
      std::make_tuple("{{<!>},{<!>},{<!>},{<a>}}", 3),
  };

  for (const auto& t : tests) {
    const auto line = std::get<0>(t);
    const auto expected = std::get<1>(t);
    const auto actual = parse(line);
    if (actual == expected) {
      std::cout << line << " works" << std::endl;
    } else {
      std::cout << line << " expected " << expected << " got " << actual
                << std::endl;
    }
  }
}

int main(int argc, char** argv) {
  if (std::string(argv[1]) == "test") {
    test();
    return 0;
  }

  std::ifstream input_f(argv[1]);
  std::string line;
  std::getline(input_f, line);

  std::cout << parse(line) << std::endl;
}
