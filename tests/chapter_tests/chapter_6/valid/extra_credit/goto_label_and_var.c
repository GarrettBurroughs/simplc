// test-directive valid
// test-directive return_code: 5
// test-directive extra_credit: goto

int main(void) {
    // it's valid to use the same identifier as a variable and label
    int ident = 5;
    goto ident;
    return 0;
ident:
    return ident;
}