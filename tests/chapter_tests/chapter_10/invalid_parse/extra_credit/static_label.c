// test-directive invalid
// test-directive extra_credit: goto

// The static specifier cannot be applied to labels

int main(void) {
    static a:
    return 1;
}