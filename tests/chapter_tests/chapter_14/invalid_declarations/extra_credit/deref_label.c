// test-directive invalid
// test-directive extra_credit: goto

// It's illegal to dereference a label
int main(void) {
    lbl:
    *lbl;
    return 0;
}