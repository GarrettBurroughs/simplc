// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: goto

/* The same identifier can be used
 * in the same scope as both a function name and a label */
int foo(void) {
    goto foo;
    return 0;
    foo:
        return 1;
}

int main(void) {
    return foo();
}