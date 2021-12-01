#include <iostream>

template <typename T>
struct node {
 public:
  struct node<T>* next = nullptr;
  int id;
  T value;

  node(int ident, T val) : id(ident), value(val) {}

  void removeNext() {
    if (!next) {
      return;
    }
    auto nextptr = next;
    next = next->next;
    delete nextptr;
  }

  void addNext(int ident, T val) { next = new node<T>(ident, val); }
};

void take_gifts(node<int>* iter) {
  iter->value += iter->next->value;
  iter->removeNext();
}

node<int>* init_elves(int count) {
  node<int>* head = new node<int>(1, 1);
  auto iter = head;
  for (int i = 2; i <= count; i++) {
    iter->addNext(i, 1);
    iter = iter->next;
  }
  iter->next = head;
  return head;
}

int main(int argc, char** argv) {
  const int elf_count = 3018458;
  auto iter = init_elves(elf_count);
  int prev_id = -1;
  while (iter) {
    if (iter->id == prev_id) {
      std::cout << iter->id << std::endl;
      return 0;
    }
    take_gifts(iter);
    prev_id = iter->id;
    iter = iter->next;
  }
}
