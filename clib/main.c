#include <stdint.h>
#include "../rlib/rlib.h"

struct Buffer poll() {
  struct Buffer b;
  b.len = 1;
  b.data = 1;
  return b;
}