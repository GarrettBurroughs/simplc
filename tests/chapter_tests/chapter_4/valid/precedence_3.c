// test-directive valid
// test-directive return_code: 0

#ifdef SUPPRESS_WARNINGS
#ifndef __clang__
#pragma GCC diagnostic ignored "-Wparentheses"
#endif
#endif
int main(void) {
    return 2 == 2 >= 0;
}