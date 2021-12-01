#include <fstream>
#include <iostream>
#include <queue>
#include <tuple>
#include <unordered_map>

class Room {
 private:
  const std::tuple<bool, int> is_valid_;

  const std::tuple<bool, int> is_valid_room(const std::string& line) {
    //std::cout << line << std::endl;
    std::unordered_map<char, int> chars;
    for (const char c : line) {
      if (c == '[') {
        break;
      }
      if (c < 97 || c > 122) {
        continue;
      }
      chars.try_emplace(c, 0);
      chars[c]++;
    }
    auto open_bracket = line.find('[');
    std::string checksum{line.begin() + open_bracket + 1, line.begin() + line.find(']')};
    std::string num_s{line.begin() + line.find_last_of('-') + 1, line.begin() + open_bracket};
    int number = std::stoi(num_s);

    auto cmp = [](std::tuple<char, int> left, std::tuple<char, int> right) {
      if (std::get<1>(left) == std::get<1>(right)) {
        return std::get<0>(left) > std::get<0>(right);
      }
      return std::get<1>(left) < std::get<1>(right);
    };
    std::priority_queue<std::tuple<char, int>, std::vector<std::tuple<char, int>>, decltype(cmp)> freqs(cmp);
    for (const auto& p : chars) {
      freqs.push(std::make_tuple(p.first, p.second));
    }

    for (int i = 0; i < 5; i++) {
      auto t = freqs.top();
      freqs.pop();
      if (std::get<0>(t) != checksum[i]) {
        return std::make_tuple(false, number);
      }
    }

    return std::make_tuple(true, number);
  }

 public:
  const bool valid_room;
  const int number;

  Room(const std::string& line) : is_valid_(is_valid_room(line)), valid_room(std::get<0>(is_valid_)),
    number(std::get<1>(is_valid_)) {}
};

const std::vector<Room> read_rooms(const std::string& path) {
  std::ifstream room_f(path);

  std::vector<Room> rooms;
  if (!room_f.is_open()) {
    std::cerr << "Unable to open rooms file \"" << path << "\"" << std::endl;
    return rooms;
  }

  std::string line;
  while (std::getline(room_f, line)) {
    rooms.emplace_back(line);
  }

  return rooms;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  const auto rooms = read_rooms(argv[1]);

  int sum = 0;
  for (const auto& room : rooms) {
    if (room.valid_room) {
      sum += room.number;
    }
  }

  std::cout << sum << std::endl;
}
