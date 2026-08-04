// test-directive invalid

int foo(void);

int foo(void) {
    return 1;
}

int foo(void) {
    return 2;
}

int main(void) {
    return foo();
}
