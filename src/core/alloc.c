#include <types.h>
#include <std/stdlib.h>
#include <core/alloc.h>

static struct {
  alloc_memory_pool pool;
  void* current_pool_ptr;
} CURRENT_ALLOC_CTX;

static u8 initialized_alloc = 0;

void alloc_init(alloc_memory_pool pool)
{
  CURRENT_ALLOC_CTX.pool = pool;
  CURRENT_ALLOC_CTX.current_pool_ptr = (void*)(((u32)pool.start + 7) & ~7);
  if (CURRENT_ALLOC_CTX.current_pool_ptr <= CURRENT_ALLOC_CTX.pool.end) {
    initialized_alloc = 1;
  }
}

void* alloc(size count)
{
  if (!initialized_alloc) return NULL;
  void* new_pool_ptr = (void*)((((u32)CURRENT_ALLOC_CTX.current_pool_ptr + 7) & ~7) + count);
  if (new_pool_ptr > CURRENT_ALLOC_CTX.pool.end) return NULL;
  void* old_pool_ptr = CURRENT_ALLOC_CTX.current_pool_ptr;
  CURRENT_ALLOC_CTX.current_pool_ptr = new_pool_ptr;
  return old_pool_ptr;
}

void free(void* ptr) { UNUSED(ptr); }
