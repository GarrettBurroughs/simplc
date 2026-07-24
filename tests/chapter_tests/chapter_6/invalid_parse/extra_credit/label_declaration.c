// test-directive invalid
// test-directive extra_credit: goto

int main(void) {
// NOTE: this is a syntax error in C17 but valid in C23
label:
    int a = 0;
    return 0;
}