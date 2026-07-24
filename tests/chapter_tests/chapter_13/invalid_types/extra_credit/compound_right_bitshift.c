// test-directive invalid
// test-directive extra_credit: bitwise, compound

int main(void) {
    // Can't perform compound bitwise operations with doubles
    int i = 1000;
    i >>= 2.0;
    return i;
}