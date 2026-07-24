// test-directive valid
// test-directive return_code: 0
// test-directive extra_credit: switch

#ifdef SUPPRESS_WARNINGS
#ifndef __clang__
#pragma GCC diagnostic ignored "-Wimplicit-fallthrough"
#endif
#endif

// test that we can fall through from default to other cases
// if default isn't last
int main(void) {
    int a = 5;
    switch(0) {
        default:
            a = 0;
        case 1:
            return a;
    }
    return a + 1;
}