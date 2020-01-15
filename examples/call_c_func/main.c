#include <stdint.h>
#include <stdlib.h>
#include "./example.h"

struct bug79096 bug79096(void)
{
  struct bug79096 b;

  b.a = 1;
  b.b = 1;
  return b;
}