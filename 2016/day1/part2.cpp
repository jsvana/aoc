#include <cmath>
#include <fstream>
#include <iostream>
#include <set>
#include <tuple>
#include <vector>

const std::vector<std::tuple<char, int>> read_moves(const std::string& path) {
  std::ifstream moves_f(path);

  std::vector<std::tuple<char, int>> moves;
  if (!moves_f.is_open()) {
    std::cerr << "Unable to open moves file \"" << path << "\"" << std::endl;
    return moves;
  }

  std::string line;
  std::getline(moves_f, line);

  std::size_t start = 0, end = 0;
  while (end != std::string::npos) {
    end = line.find(", ", start);
    auto len = end;
    if (end == std::string::npos) {
      len = line.length();
    }
    moves.push_back(std::make_tuple(line[start], std::stoi(line.substr(start + 1, len - start - 1))));
    start = len + 2;
  }

  return moves;
}

const std::tuple<int, int> find_end(const std::vector<std::tuple<char, int>>& moves) {
  int direction = 0; // 0 = N, 1 = E, 2 = S, 3 = W
  int x = 0, y = 0;

  std::set<std::tuple<int, int>> visited;

  for (const auto& m : moves) {
    if (std::get<0>(m) == 'R') {
      direction++;
      if (direction > 3) {
        direction = 0;
      }
    } else {
      direction--;
      if (direction < 0) {
        direction = 3;
      }
    }

    int magnitude = std::get<1>(m);
    int old_x = x, old_y = y;
    switch (direction) {
    case 0:
      y -= magnitude;
      for (int i = old_y - 1; i >= y; i--) {
        auto pos = std::make_tuple(x, i);
        if (visited.find(pos) == visited.end()) {
          visited.insert(pos);
        } else {
          return pos;
        }
      }
      break;
    case 1:
      x += magnitude;
      for (int i = old_x + 1; i <= x; i++) {
        auto pos = std::make_tuple(i, y);
        if (visited.find(pos) == visited.end()) {
          visited.insert(pos);
        } else {
          return pos;
        }
      }
      break;
    case 2:
      y += magnitude;
      for (int i = old_y + 1; i <= y; i++) {
        auto pos = std::make_tuple(x, i);
        if (visited.find(pos) == visited.end()) {
          visited.insert(pos);
        } else {
          return pos;
        }
      }
      break;
    case 3:
      x -= magnitude;
      for (int i = old_x - 1; i >= x; i--) {
        auto pos = std::make_tuple(i, y);
        if (visited.find(pos) == visited.end()) {
          visited.insert(pos);
        } else {
          return pos;
        }
      }
      break;
    }
  }

  return std::make_tuple(x, y);
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  const auto end = find_end(read_moves(argv[1]));
  int blocks = abs(std::get<0>(end)) + abs(std::get<1>(end));

  std::cout << "First double is " << blocks << " blocks away" << std::endl;
}
