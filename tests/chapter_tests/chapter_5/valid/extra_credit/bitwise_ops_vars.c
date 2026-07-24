// test-directive valid
// test-directive return_code: 9
// test-directive extra_credit: bitwise

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wparentheses"
#endif

int main(void) {
    int a = 3;
    int b = 5;
    int c = 8;
    return a & b | c;
}