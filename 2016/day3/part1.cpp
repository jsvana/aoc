#include <fstream>
#include <iostream>
#include <sstream>

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

const int count_valid_triangles(const std::string& path) {
  std::ifstream triangle_f(path);

  if (!triangle_f.is_open()) {
    std::cerr << "Unable to open triangles file \"" << path << "\"" << std::endl;
    return 0;
  }

  int valid = 0;
  std::string line;
  while (std::getline(triangle_f, line)) {
    std::stringstream line_s{line};
    int s1, s2, s3;
    line_s >> s1;
    line_s >> s2;
    line_s >> s3;

    if (valid_triangle(s1, s2, s3)) {
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

  std::cout << count_valid_triangles(argv[1]) << " valid triangles" << std::endl;
}
