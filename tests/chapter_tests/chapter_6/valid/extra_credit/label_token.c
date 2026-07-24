// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: goto

int main(void) {
    goto _foo_1_;  // a label may include numbers and underscores
    return 0;
_foo_1_:
    return 1;
}