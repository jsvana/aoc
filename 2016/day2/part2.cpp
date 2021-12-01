#include <algorithm>
#include <cmath>
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
const int get_min(const int val) {
  return abs(val - (int)N / 2);
}

template<size_t N>
const int get_max(const int val) {
  return abs((int)N - get_min<N>(val) - 1);
}

template<size_t N>
const std::tuple<int, int> find_end(const std::array<std::array<char, N>, N>& board, const std::tuple<int, int>& position, const std::vector<char>& moves) {
  int x = std::get<0>(position);
  int y = std::get<1>(position);

  for (const char move : moves) {
    switch (move) {
    case 'U':
      y--;
      clamp<int>(y, get_min<N>(x), get_max<N>(x));
      break;
    case 'D':
      y++;
      clamp<int>(y, get_min<N>(x), get_max<N>(x));
      break;
    case 'L':
      x--;
      clamp<int>(x, get_min<N>(y), get_max<N>(y));
      break;
    case 'R':
      x++;
      clamp<int>(x, get_min<N>(y), get_max<N>(y));
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

  std::array<std::array<char, 5>, 5> board = {{
    {'\0', '\0', '1', '\0', '\0'},
    {'\0',  '2', '3',  '4', '\0'},
    { '5',  '6', '7',  '8',  '9'},
    {'\0',  'A', '7',  '8', '\0'},
    {'\0', '\0', 'D', '\0', '\0'},
  }};

  const auto moves = read_moves(argv[1]);

  auto position = std::make_tuple(0, 1);

  for (const auto& line_moves : moves) {
    const auto new_position = find_end<5>(board, position, line_moves);
    std::cout << board[std::get<1>(new_position)][std::get<0>(new_position)];
  }
  std::cout << std::endl;
}
