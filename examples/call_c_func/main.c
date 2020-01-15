#include <stdint.h>
#include <stdlib.h>
#include "./example.h"

struct ComplexFlatStructure* return_pointer() {
  struct ComplexFlatStructure *b = malloc(sizeof(struct ComplexFlatStructure));
  b->_1 = 1;
  b->_2 = 1;
  b->_3 = 1;
  b->_4 = 1;
  b->_5 = 1;
  b->_6 = 1;
  b->_7 = 1;
  b->_8 = 8;
  return b;
}