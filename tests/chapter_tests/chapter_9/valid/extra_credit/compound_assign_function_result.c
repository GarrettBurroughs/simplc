// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: compound

int foo(void) {
    return 2;
}

int main(void) {
    int x = 3;
    x -= foo();
    return x;
}