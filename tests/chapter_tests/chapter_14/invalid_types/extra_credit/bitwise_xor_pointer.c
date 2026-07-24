// test-directive invalid
// test-directive extra_credit: bitwise

/* It's illegal to apply bitwise operations where either operand is a pointer */
int main(void) {
    unsigned long *ptr = 0;
    long l = 100;
    ptr ^ l;
    return 0;
}