// test-directive invalid
// test-directive extra_credit: compound

// Can't compound assign to string literal
int main(void) {
    "My string" += 1;
    return 0;
}