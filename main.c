#include <stdint.h>
#include <stdio.h>

uint32_t rust_function();

int main() {
  uint32_t x = rust_function();
  printf("%d\n", x);
  return 0;
}
