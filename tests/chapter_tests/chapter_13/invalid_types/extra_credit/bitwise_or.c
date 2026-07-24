// test-directive invalid
// test-directive extra_credit: bitwise

int main(void) {
    /* It's illegal to apply bitwise | to doubles */
    double d = 0.0 | -0.0;
    return 0;
}