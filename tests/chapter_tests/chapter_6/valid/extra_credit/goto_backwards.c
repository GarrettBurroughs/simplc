// test-directive valid
// test-directive return_code: 5
// test-directive extra_credit: goto

int main(void) {
    if (0)
    label:
        return 5;
    goto label;
    return 0;
}