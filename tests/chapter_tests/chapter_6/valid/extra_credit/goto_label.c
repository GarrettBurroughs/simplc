// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: goto

int main(void) {
    goto label;
    return 0;
label:
    return 1;
}