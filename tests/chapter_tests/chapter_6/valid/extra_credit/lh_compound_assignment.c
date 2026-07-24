// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: compound

int main(void) {
    int x = 10;
    (x -= 1) ? (x /= 2) : 0;
    return x == 4;
}