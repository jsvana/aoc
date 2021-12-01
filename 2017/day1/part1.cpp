#include <fstream>
#include <iostream>
#include <vector>

const std::vector<int> read_digits(const std::string& path) {
  std::ifstream input_f(path);

  std::vector<int> digits;
  if (!input_f.is_open()) {
    std::cerr << "Unable to open file \"" << path << "\"" << std::endl;
    return digits;
  }

  std::string line;
  std::getline(input_f, line);

  for (const char c : line) {
    digits.push_back(c - '0');
  }

  return digits;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  const auto digits = read_digits(argv[1]);
  int sum = 0;
  for (std::size_t i = 1; i < digits.size(); i++) {
    if (digits[i] == digits[i - 1]) {
      sum += digits[i];
    }
  }
  if (digits[digits.size() - 1] == digits[0]) {
    sum += digits[0];
  }
  std::cout << sum << std::endl;
}
