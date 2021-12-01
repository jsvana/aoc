#include <fstream>
#include <iostream>
#include <queue>
#include <tuple>
#include <unordered_map>

const std::vector<std::string> read_lines(const std::string& path) {
  std::ifstream lines_f(path);

  std::vector<std::string> lines;
  if (!lines_f.is_open()) {
    std::cerr << "Unable to open lines file \"" << path << "\"" << std::endl;
    return lines;
  }

  std::string line;
  while (std::getline(lines_f, line)) {
    lines.push_back(line);
  }

  return lines;
}

const std::string correct_errors(const std::vector<std::string>& lines) {
  if (lines.empty()) {
    return "";
  }

  std::vector<std::unordered_map<char, int>> chars;
  chars.reserve(lines[0].length());
  for (int i = 0; i < lines[0].length(); i++) {
    chars.emplace_back();
  }

  for (const auto& line : lines) {
    for (int i = 0; i < line.length(); i++) {
      ++chars[i].try_emplace(line[i], 0).first->second;
    }
  }

  auto cmp = [](auto a, auto b) { return std::get<1>(a) > std::get<1>(b); };
  std::string corrected(' ', chars.size());
  for (int i = 0; i < chars.size(); i++) {
    std::priority_queue<std::pair<char, int>, std::vector<std::pair<char, int>>, decltype(cmp)> freqs(cmp);
    for (const auto& p : chars[i]) {
      freqs.push(p);
    }
    corrected[i] = freqs.top().first;
  }

  return corrected;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  const auto lines = read_lines(argv[1]);
  std::cout << correct_errors(lines) << std::endl;
}
