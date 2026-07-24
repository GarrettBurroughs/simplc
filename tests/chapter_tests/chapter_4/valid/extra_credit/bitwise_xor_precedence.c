// test-directive valid
// test-directive return_code: 5
// test-directive extra_credit: bitwise

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wparentheses"
#endif

int main(void) {
    // ^ has lower precedence than <
    return 5 ^ 7 < 5;
}