// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: bitwise

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wparentheses"
#endif

int main(void) {
    return 20 >> 4 <= 3 << 1;
}