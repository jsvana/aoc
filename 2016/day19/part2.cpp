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

void take_gifts(node<int>* iter, node<int>* middle) {
  iter->value += middle->next->value;
  middle->removeNext();
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
  int elf_count = 3018458;
  // int elf_count = 5;
  auto iter = init_elves(elf_count);
  auto middle = iter;
  for (int i = 0; i < elf_count / 2 - 1; i++) {
    middle = middle->next;
  }
  // Answer too low, not sure why
  while (iter != middle) {
    take_gifts(iter, middle);
    iter = iter->next;
    if (elf_count % 2 == 0) {
      middle = middle->next;
    }
    --elf_count;
  }
  std::cout << iter->id << std::endl;
}
