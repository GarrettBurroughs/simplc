// test-directive invalid
// test-directive extra_credit: bitwise

// can't apply << or >> to strings
int main(void) {
    "foo" << 3;
    return 0;
}