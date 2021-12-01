#include <fstream>
#include <iostream>
#include <set>
#include <tuple>
#include <vector>

const std::vector<std::string> read_lines(const std::string& path) {
  std::ifstream input_f(path);

  std::vector<std::string> lines;
  if (!input_f.is_open()) {
    std::cerr << "Unable to open file \"" << path << "\"" << std::endl;
    return lines;
  }

  std::string line;
  while (std::getline(input_f, line)) {
    lines.push_back(line);
  }

  return lines;
}

const bool valid_passphrase(const std::string& passphrase) {
  std::set<std::string> words;
  std::size_t start = 0, end;
  while ((end = passphrase.find(" ", start)), end != std::string::npos) {
    const std::string word = passphrase.substr(start, end - start);
    if (words.find(word) != words.end()) {
      return false;
    }
    words.insert(word);
    start = end + 1;
  }
  const std::string last_word = passphrase.substr(passphrase.rfind(" ") + 1);
  if (words.find(last_word) != words.end()) {
    return false;
  }
  return true;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  int valid = 0;
  for (const auto& line : read_lines(argv[1])) {
    if (valid_passphrase(line)) {
      ++valid;
    }
  }

  std::cout << valid << std::endl;
}
