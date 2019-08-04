#include <iostream>

static bool will_overflow(unsigned char c) {
  return c < c - 1;
}

int main() {
  // output with -O0: will_overflow == 1 // 0 < 255
  // output with -O3: will_overflow == 0 // compiler considers that c < c - 1 is always false
  std::cout << "will_overflow == " << will_overflow(0) << std::endl;
  return 0;
}
