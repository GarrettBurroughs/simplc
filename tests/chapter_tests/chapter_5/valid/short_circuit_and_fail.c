// test-directive valid
// test-directive return_code: 0

#ifdef SUPPRESS_WARNINGS
#ifndef __clang__
#pragma GCC diagnostic ignored "-Wunused-value"
#endif
#endif

int main(void) {
    int a = 0;
    0 && (a = 5);
    return a;
}