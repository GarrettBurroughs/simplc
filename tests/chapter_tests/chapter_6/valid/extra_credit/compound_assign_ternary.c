// test-directive valid
// test-directive return_code: 8
// test-directive extra_credit: compound

int main(void) {
    int a = 4;
    a *= 1 ? 2 : 3;
    return a;
}
