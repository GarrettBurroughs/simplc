// test-directive valid
// test-directive return_code: 0
// test-directive extra_credit: goto

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wunused-label"
#endif

int main(void) {
unused:
    return 0;
}