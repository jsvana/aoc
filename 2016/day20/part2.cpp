#include <algorithm>
#include <fstream>
#include <iostream>
#include <queue>
#include <utility>
#include <vector>

typedef std::pair<long, long> Interval;

const std::vector<Interval> read_lines(const std::string& path) {
  std::ifstream lines_f(path);

  std::vector<Interval> intervals;
  if (!lines_f.is_open()) {
    std::cerr << "Unable to open lines file \"" << path << "\"" << std::endl;
    return intervals;
  }

  std::string line;
  while (std::getline(lines_f, line)) {
    intervals.push_back(std::make_pair(
        std::stol(line.data()), std::stol(line.data() + line.find('-') + 1)));
  }
  return intervals;
}

const std::vector<Interval> merge(std::vector<Interval>& intervals) {
  std::vector<Interval> merged;
  std::sort(intervals.begin(), intervals.end());
  auto start = intervals[0].first;
  auto end = intervals[0].second;
  for (const auto& interval : intervals) {
    if (interval.first <= end) {
      end = std::max(interval.second, end);
    } else {
      merged.push_back(std::make_pair(start, end));
      start = interval.first;
      end = interval.second;
    }
  }
  merged.push_back(std::make_pair(start, end));
  return merged;
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cerr << "Usage: " << argv[0] << " <input>" << std::endl;
    return 1;
  }

  auto intervals = read_lines(argv[1]);

  const auto merged = merge(intervals);
  auto count = std::max(merged[0].first - 1, 0l);
  auto end = merged[0].second;
  for (const auto& interval : merged) {
    count += std::max(interval.first - end - 1, 0l);
    end = interval.second;
  }
  std::cout << count << std::endl;
}
