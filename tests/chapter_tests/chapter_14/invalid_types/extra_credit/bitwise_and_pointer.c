// test-directive invalid
// test-directive extra_credit: bitwise

/* It's illegal to apply bitwise operations where either operand is a pointer */
int main(void) {
    long *ptr = 0;
    10 & ptr;
    return 0;
}