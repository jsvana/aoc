#include <fstream>
#include <iostream>
#include <queue>
#include <tuple>
#include <unordered_map>

enum class CommandType : int {
  RECT = 0,
  ROTATE_X = 1,
  ROTATE_Y = 2,
};

struct Command {
 private:
  const std::tuple<CommandType, int, int> parts_;

  const std::tuple<int, int> get_rect_inputs(const std::string& line) {
    return std::make_tuple(std::stoi(line), std::stoi(line.data() + line.find('x') + 1));
  }

  const std::tuple<int, int> get_rotate_inputs(const std::string& line) {
    auto start = line.find('=') + 1;
    int one = std::stoi(line.data() + start);
    int two = std::stoi(line.data() + line.find(' ', line.find(' ', start) + 1) + 1);
    return std::make_tuple(one, two);
  }

  const std::tuple<CommandType, int, int> get_parts(const std::string& line) {
    auto end = line.find(' ');
    auto start = end + 1;
    auto cmd_type = line.substr(0, end);
    CommandType type;
    std::tuple<int, int> inputs;
    if (cmd_type == "rect") {
      type = CommandType::RECT;
      inputs = get_rect_inputs(line.substr(start));
    } else {
      end = line.find(' ', end + 1);
      auto subcmd = line.substr(start, end - start);
      if (subcmd == "column") {
        type = CommandType::ROTATE_Y;
      } else {
        type = CommandType::ROTATE_X;
      }
      inputs = get_rotate_inputs(line.substr(end + 1));
    }

    return std::make_tuple(type, std::get<0>(inputs), std::get<1>(inputs));
  }

 public:
  const CommandType type;
  const int input1;
  const int input2;

  Command(const std::string& line) : parts_(get_parts(line)), type(std::get<0>(parts_)),
    input1(std::get<1>(parts_)), input2(std::get<2>(parts_)) {}
};

const std::vector<Command> read_commands(const std::string& path) {
  std::ifstream lines_f(path);

  std::vector<Command> lines;
  if (!lines_f.is_open()) {
    std::cerr << "Unable to open lines file \"" << path << "\"" << std::endl;
    return lines;
  }

  std::string line;
  while (std::getline(lines_f, line)) {
    lines.emplace_back(line);
  }

  return lines;
}

template<size_t X, size_t Y>
const std::array<std::array<bool, X>, Y> make_map() {
  std::array<std::array<bool, X>, Y> arr;
  for (auto& line : arr) {
    line.fill(false);
  }
  return arr;
}

template<size_t X, size_t Y>
void print_map(const std::array<std::array<bool, X>, Y>& map) {
  for (int i = 0; i < Y; i++) {
    for (int j = 0; j < X; j++) {
      std::cout << (map[i][j] ? '#' : '.');
    }
    std::cout << std::endl;
  }
}

template<size_t X, size_t Y>
void run_command(std::array<std::array<bool, X>, Y>& map, const Command cmd) {
  std::vector<bool> new_line;
  switch (cmd.type) {
  case CommandType::RECT:
    for (int i = 0; i < cmd.input2; i++) {
      for (int j = 0; j < cmd.input1; j++) {
        map[i][j] = true;
      }
    }
    break;

  case CommandType::ROTATE_X:
    new_line.reserve(X + cmd.input2);
    for (int i = 0; i < X; i++) {
      new_line[i + cmd.input2] = map[cmd.input1][i];
    }
    for (int i = 0; i < X; i++) {
      map[cmd.input1][(i + cmd.input2) % X] = new_line[i + cmd.input2];
    }
    break;

  case CommandType::ROTATE_Y:
    new_line.reserve(Y + cmd.input2);
    for (int i = 0; i < Y; i++) {
      new_line[i + cmd.input2] = map[i][cmd.input1];
    }
    for (int i = 0; i < Y; i++) {
      map[(i + cmd.input2) % Y][cmd.input1] = new_line[i + cmd.input2];
    }
    break;
  }
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  const auto commands = read_commands(argv[1]);
  auto map = make_map<50, 6>();

  for (const auto& command : commands) {
    run_command(map, command);
  }

  int count = 0;
  for (int i = 0; i < 6; i++) {
    for (int j = 0; j < 50; j++) {
      if (map[i][j]) {
        ++count;
      }
    }
  }
  std::cout << count << std::endl;
}
