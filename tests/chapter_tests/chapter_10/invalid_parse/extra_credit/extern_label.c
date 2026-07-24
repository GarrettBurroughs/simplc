// test-directive invalid
// test-directive extra_credit: goto

// The extern specifier cannot be applied to labels

int main(void) {
    extern a:
    return 1;
}