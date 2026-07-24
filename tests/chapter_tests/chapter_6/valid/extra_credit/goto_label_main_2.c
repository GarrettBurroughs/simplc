// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: goto

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wunused-label"
#endif
int main(void) {
    goto _main;
    return 0;
    _main:
        return 1;
}