/* ===================================================== **
**  This file is a part of the via Programming Language  **
** ----------------------------------------------------- **
**           Copyright (C) XnLogicaL 2024-2025           **
**              Licensed under GNU GPLv3.0               **
** ----------------------------------------------------- **
**         https://github.com/XnLogicaL/via-lang         **
** ===================================================== */

#include "memory.hpp"

#include <mimalloc.h>

void via::ObjectTracker::clear() {
  for (auto it = m_tracker.rbegin(); it != m_tracker.rend(); ++it) {
    if (!it->second.destroyed) {
      it->second.destroy(it->first, it->second.count);
      it->second.destroyed = true;
    }
  }
  m_tracker.clear();
}

via::ScopedAllocator::ScopedAllocator() : m_heap(mi_heap_new()) {}

via::ScopedAllocator::~ScopedAllocator() {
  m_tracker.clear();

  if (m_heap) {
    // TODO: Uncomment when fixed
    // Causes UB due to an internal bug in mimalloc
    // See: https://github.com/microsoft/mimalloc/issues/1146
    //
    // mi_heap_destroy(m_heap);
    m_heap = nullptr;
  }
}

bool via::ScopedAllocator::owns(void* ptr) {
  return m_heap && mi_heap_check_owned((mi_heap_t*)m_heap, ptr);
}

void* via::ScopedAllocator::alloc(size_t size) {
  return mi_heap_malloc((mi_heap_t*)m_heap, size);
}

char* via::ScopedAllocator::strdup(const char* str) {
  return mi_heap_strdup((mi_heap_t*)m_heap, str);
}

char* via::ScopedAllocator::strndup(const char* str, size_t n) {
  return mi_heap_strndup((mi_heap_t*)m_heap, str, n);
}

void via::ScopedAllocator::free(void* ptr) {
  VIA_DEBUG_ASSERT(owns(ptr),
                   std::format("free() called on address {:p} which is not "
                               "owned by <ScopedAllocator@{:p}>",
                               (const void*)ptr, (const void*)this));
  m_tracker.delete_at(ptr);
  mi_free(ptr);
}
