#include <algorithm>
#include <fstream>
#include <iostream>
#include <tuple>
#include <vector>

template<typename T>
void clamp(T& val, T min, T max) {
  val = std::min(std::max(val, min), max);
}

const std::vector<std::vector<char>> read_moves(const std::string& path) {
  std::ifstream moves_f(path);

  std::vector<std::vector<char>> moves;
  if (!moves_f.is_open()) {
    std::cerr << "Unable to open moves file \"" << path << "\"" << std::endl;
    return moves;
  }

  std::string line;
  while (std::getline(moves_f, line)) {
    moves.push_back({line.begin(), line.end()});
  }

  return moves;
}

template<size_t N>
const std::tuple<int, int> find_end(const std::array<std::array<int, N>, N>& board, const std::tuple<int, int>& position, const std::vector<char>& moves) {
  int x = std::get<0>(position);
  int y = std::get<1>(position);

  for (const char move : moves) {
    switch (move) {
    case 'U':
      y--;
      clamp<int>(y, 0, N - 1);
      break;
    case 'D':
      y++;
      clamp<int>(y, 0, N - 1);
      break;
    case 'L':
      x--;
      clamp<int>(x, 0, N - 1);
      break;
    case 'R':
      x++;
      clamp<int>(x, 0, N - 1);
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

  std::array<std::array<int, 3>, 3> board = {{
    {1, 2, 3},
    {4, 5, 6},
    {7, 8, 9},
  }};

  const auto moves = read_moves(argv[1]);

  auto position = std::make_tuple(1, 1);

  for (const auto& line_moves : moves) {
    const auto new_position = find_end<3>(board, position, line_moves);
    std::cout << board[std::get<1>(new_position)][std::get<0>(new_position)];
  }
  std::cout << std::endl;
}
