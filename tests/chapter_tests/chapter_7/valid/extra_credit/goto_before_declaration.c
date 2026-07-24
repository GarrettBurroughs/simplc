// test-directive valid
// test-directive return_code: 0
// test-directive extra_credit: goto

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wunused-variable"
#endif
int main(void) {
    int a = 0;
    {
        if (a != 0)
            return_a:
                return a;
        int a = 4;
        goto return_a;
    }
}