// test-directive valid
// test-directive return_code: 5
// test-directive extra_credit: bitwise

int main(void) {
    int result;
    1 ^ 1 ? result = 4 : (result = 5);
    return result;
}