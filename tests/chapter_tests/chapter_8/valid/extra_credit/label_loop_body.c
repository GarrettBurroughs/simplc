// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: goto

// a loop body may be a labeled statement
int main(void) {
    int result = 0;
    goto label;
    while (0)
    label: { result = 1; }

    return result;
}