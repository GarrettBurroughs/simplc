// test-directive valid
// test-directive return_code: 0
// test-directive include pass_alloced_memory_client.c

void *calloc(unsigned long nmemb, unsigned long size);
void *memset(void *s, int c, unsigned long n);
void free(void *ptr);

void *get_100_zeroed_bytes(void) {
    return calloc(100, 1);
}

void fill_100_bytes(void *pointer, int byte) {
    memset(pointer, byte, 100);
}

void free_bytes(void *ptr) {
    free(ptr);
}