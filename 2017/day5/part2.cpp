#include <fstream>
#include <iostream>
#include <set>
#include <tuple>
#include <vector>

const std::vector<int> read_lines(const std::string& path) {
  std::ifstream input_f(path);

  std::vector<int> lines;
  if (!input_f.is_open()) {
    std::cerr << "Unable to open file \"" << path << "\"" << std::endl;
    return lines;
  }

  std::string line;
  while (std::getline(input_f, line)) {
    lines.push_back(std::atoi(line.c_str()));
  }

  return lines;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  auto instructions = read_lines(argv[1]);
  int i = 0;
  int count = 0;
  while (i >= 0 && i < instructions.size()) {
    int new_i = i + instructions[i];
    if (instructions[i] >= 3) {
      --instructions[i];
    } else {
      ++instructions[i];
    }
    ++count;
    i = new_i;
  }
  std::cout << count << std::endl;
}
