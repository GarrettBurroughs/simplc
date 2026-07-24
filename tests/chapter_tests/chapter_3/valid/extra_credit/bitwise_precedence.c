// test-directive valid
// test-directive return_code: 21
// test-directive extra_credit: bitwise

#ifdef SUPPRESS_WARNINGS
#ifdef __clang__
#pragma clang diagnostic ignored "-Wbitwise-op-parentheses"
#else
#pragma GCC diagnostic ignored "-Wparentheses"
#endif
#endif

int main(void) {
    return 80 >> 2 | 1 ^ 5 & 7 << 1;
}