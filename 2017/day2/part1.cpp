#include <algorithm>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

const std::vector<std::vector<int>> read_spreadsheet(const std::string& path) {
  std::ifstream input_f(path);

  std::vector<std::vector<int>> rows;
  if (!input_f.is_open()) {
    std::cerr << "Unable to open file \"" << path << "\"" << std::endl;
    return rows;
  }

  std::string line;
  while (std::getline(input_f, line)) {
    rows.emplace_back();
    int item;
    std::istringstream line_stream(line);
    while (line_stream >> item) {
      rows.back().push_back(item);
    }
  }

  return rows;
}

const int row_difference(const std::vector<int>& row) {
  const auto res = std::minmax_element(row.begin(), row.end());
  return *res.second - *res.first;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  const auto rows = read_spreadsheet(argv[1]);
  int sum = 0;
  for (const auto& row : rows) {
    const auto difference = row_difference(row);
    sum += difference;
  }
  std::cout << sum << std::endl;
}
