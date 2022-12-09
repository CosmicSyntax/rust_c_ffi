#include <stdint.h>
#include <stdio.h>
#include "rust/rust.h"

int main() {
  uint32_t x = rust_function(100);
  printf("%d\n", x);
  return 0;
}
