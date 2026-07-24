// test-directive valid
// test-directive return_code: 77
// test-directive extra_credit: bitwise

int main(void) {
    int var_to_shift = 1234;
    int x = 0;
    x = var_to_shift >> 4;
    return x;
}