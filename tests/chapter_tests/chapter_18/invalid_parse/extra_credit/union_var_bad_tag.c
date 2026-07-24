// test-directive invalid
// test-directive extra_credit: union

int main(void) {
    union 4 foo;  // a union tag must be an identifier (not a constant)
    return 0;
}