#include <fstream>
#include <iostream>
#include <queue>
#include <stack>
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

const bool supports_tls(const std::string& address) {
  bool in_bracket = false;
  std::stack<char> chars;
  char last_char = 0;

  bool found = false;
  bool valid = false;
  for (const char c : address) {
    if (c == '[') {
      while (!chars.empty()) {
        chars.pop();
      }
      last_char = 0;
      in_bracket = true;
      continue;
    } else if (c == ']') {
      while (!chars.empty()) {
        chars.pop();
      }
      last_char = 0;
      in_bracket = false;
      continue;
    }

    if (!chars.empty() && c == chars.top()) {
      if (last_char != 0) {
        if (in_bracket) {
          return false;
        } else {
          valid = true;
        }
      } else {
        last_char = c;
        chars.pop();
      }
    } else {
      last_char = 0;
      chars.push(c);
    }
  }

  return valid;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  const std::array<std::tuple<std::string, bool>, 21> tests = {
    std::make_tuple("abba[mnop]qrst", true),
    std::make_tuple("abcd[bddb]xyyx", false),
    std::make_tuple("aaaa[qwer]tyui", false),
    std::make_tuple("ioxxoj[asdfgh]zxcvbn", true),
    std::make_tuple("ab[asdf]ba", false),
    std::make_tuple("aba[foobar]asdf", false),
    std::make_tuple("ab[bafoobar]asdf", false),
    std::make_tuple("ab[bafoobar]abba[asdf]asdf", true),
    std::make_tuple("ab[ba]abba[a]a", true),
    std::make_tuple("abba[qwwq]", false),
    std::make_tuple("[abba]qwwq", false),
    std::make_tuple("abba[abba]qwwq", false),
    std::make_tuple("as[abba]df", false),
    std::make_tuple("as[aaaa]df", false),
    std::make_tuple("c[d]abba[e]f", true),
    std::make_tuple("c[d]abba[asdf][e]f", true),
    // Fails the following 5
    std::make_tuple("abudxncgozbrbnx[fllpjgocynbuyawgs]hiphrvpugpfnnppn[jhmlgjsufflkdgw]ldmdclrkorzjtbjqcrn", true),
    std::make_tuple("lwnhrcbjrjqarzdx[jezkqlffqqbioajjbnl]zssjjsdouwbegdbnxx", true),
    std::make_tuple("urlkduvyyyatpkb[zrolecowduswyfn]bgkveercmmeecop", false),
    std::make_tuple("ekecthrkwdbjhsy[klxbdnucasemwhlpjj]jbvenwrnvynlfyjybm[zgfxxurrduhtlmsbelf]lxuxlahnrqvjssffj[wzcpjiesgsbwbtnlrs]sofzsskbviyfvlo", false),
    std::make_tuple("wpvcqnrvyjvfkfpclz[wogcckufvzviggf]oulptksetgaaholu[dwwcwhkktrhgkahbs]sobrvezzrrzvlihicw", true),
  };

  for (const auto& p : tests) {
    const auto line = std::get<0>(p);
    bool expected = std::get<1>(p);
    if (supports_tls(line) == expected) {
      std::cout << "passed " << line << std::endl;
    } else {
      std::cout << "failed " << line << " (expected " << expected << " but got " << !expected << ")" << std::endl;
    }
  }

  const auto addresses = read_lines(argv[1]);

  int tls = 0;
  for (const auto& address : addresses) {
    if (supports_tls(address)) {
      ++tls;
    }
  }
  std::cout << tls << std::endl;
}
