#include <array>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

const bool valid_triangle(const int s1, const int s2, const int s3) {
  if (s1 + s2 <= s3) {
    return false;
  } else if (s1 + s3 <= s2) {
    return false;
  } else if (s2 + s3 <= s1) {
    return false;
  }
  return true;
}

const std::vector<std::array<int, 3>> get_lengths(const std::string& path) {
  std::ifstream triangle_f(path);

  std::vector<std::array<int, 3>> triangles;
  if (!triangle_f.is_open()) {
    std::cerr << "Unable to open triangles file \"" << path << "\"" << std::endl;
    return triangles;
  }

  std::string line;
  while (std::getline(triangle_f, line)) {
    std::stringstream line_s{line};
    std::array<int, 3> lengths;

    for (int i = 0; i < 3; i++) {
      line_s >> lengths[i];
    }
    triangles.push_back(lengths);
  }

  return triangles;
}

const int get_length(const std::vector<std::array<int, 3>>& triangles, std::size_t& x, std::size_t& y) {
  int length = triangles[y][x];
  y++;
  if (y == triangles.size()) {
    y = 0;
    x++;
  }
  return length;
}

const int count_valid_triangles(const std::vector<std::array<int, 3>>& triangles) {
  std::size_t x = 0, y = 0;
  int valid = 0;
  while (x < 3 && y < triangles.size()) {
    if (valid_triangle(get_length(triangles, x, y), get_length(triangles, x, y), get_length(triangles, x, y))) {
      valid++;
    }
  }
  return valid;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  std::cout << count_valid_triangles(get_lengths(argv[1])) << " valid triangles" << std::endl;
}
