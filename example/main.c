#include <stdio.h>
#include <stdlib.h>
#include <sys/mman.h>
#include <unistd.h>

#define PAGE_SIZE 4096
#define NUM_PAGES 10

static void wait_for_enter(const char *msg) {
  printf("%s (press Enter to continue)\n", msg);
  getchar();
}

int main() {
  // Reserve memory with mmap
  void *addr = mmap(NULL, NUM_PAGES * PAGE_SIZE, PROT_READ | PROT_WRITE,
                    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
  if (addr == MAP_FAILED) {
    perror("mmap");
    return 1;
  }

  printf("Allocated %d pages starting at virtual address %p\n", NUM_PAGES,
         addr);

  wait_for_enter("Memory reserved but not touched yet");

  // Touch each page to force physical allocation
  for (int i = 0; i < NUM_PAGES; i++) {
    char *page_addr = (char *)addr + i * PAGE_SIZE;
    page_addr[0] = 42; // trigger page fault -> physical memory
  }

  wait_for_enter("Pages touched and should now be mapped to physical memory");

  munmap(addr, NUM_PAGES * PAGE_SIZE);
  return 0;
}
