#include "rust/rust.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
  uint32_t *x = malloc(sizeof(uint32_t));
  *x = rust_function(100);
  printf("%d\n", *x);
  free(x);
  return 0;
}
