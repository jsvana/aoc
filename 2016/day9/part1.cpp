#include <algorithm>
#include <fstream>
#include <iostream>
#include <queue>
#include <tuple>
#include <unordered_map>

const std::string read_line(const std::string& path) {
  std::ifstream lines_f(path);

  if (!lines_f.is_open()) {
    std::cerr << "Unable to open lines file \"" << path << "\"" << std::endl;
    return "";
  }

  std::string line;
  std::getline(lines_f, line);
  return line;
}

const unsigned long decompress_length(const std::string& line) {
  unsigned long len = 0;
  for (int i = 0; i < line.length(); i++) {
    if (line[i] == '(') {
      int count = std::stoi(line.data() + i + 1);
      int repeats = std::stoi(line.data() + line.find('x', i) + 1);
      auto end = line.find(')', i);
      len += count * repeats;
      i = end + count;
    } else {
      auto end = line.find('(', i);
      if (end == std::string::npos) {
        end = line.length();
      }
      len += end - i;
      i = end - 1;
    }
  }

  return len;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  const std::array<std::tuple<std::string, unsigned long>, 6> tests = {
    std::make_tuple("ADVENT", 6),
    std::make_tuple("A(1x5)BC", 7),
    std::make_tuple("(3x3)XYZ", 9),
    std::make_tuple("A(2x2)BCD(2x2)EFG", 11),
    std::make_tuple("(6x1)(1x3)A", 6),
    std::make_tuple("X(8x2)(3x3)ABCY", 18),
  };

  for (const auto& p : tests) {
    const auto line = std::get<0>(p);
    const auto expected = std::get<1>(p);
    const auto result = decompress_length(line);
    if (result == expected) {
      std::cout << "passed " << line << std::endl;
    } else {
      std::cout << "failed " << line << " (expected " << expected << " but got " << result << ")" << std::endl;
    }
  }

  const auto line = read_line(argv[1]);
  std::cout << decompress_length(line) << std::endl;
  // 102239
}
