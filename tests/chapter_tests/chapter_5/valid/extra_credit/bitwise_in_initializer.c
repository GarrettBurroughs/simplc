// test-directive valid
// test-directive return_code: 11
// test-directive extra_credit: bitwise

int main(void) {
    int a = 15;
    int b = a ^ 5;  // 10
    return 1 | b;   // 11
}