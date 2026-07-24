// test-directive valid
// test-directive return_code: 5
// test-directive extra_credit: goto

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wunused-label"
#endif
int main(void) {
    goto labelB;

    labelA:
        labelB:
            return 5;
    return 0;
}