// test-directive invalid
// test-directive extra_credit: goto

int x = 10;

int main(void) {
    /* goto statements can only target labels, not variables. */
    goto x;
    return 0;
}