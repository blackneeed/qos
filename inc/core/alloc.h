#ifndef ALLOC_H
#define ALLOC_H
#include <types.h>

typedef struct alloc_memory_pool {
  void* start;
  void* end;
} alloc_memory_pool;

void alloc_init(alloc_memory_pool pool);
void* alloc(size count);
void free(void* ptr);
#endif
