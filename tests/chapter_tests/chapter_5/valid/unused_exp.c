// test-directive valid
// test-directive return_code: 0

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wunused-value"
#endif
int main(void) {
    2 + 2;
    return 0;
}