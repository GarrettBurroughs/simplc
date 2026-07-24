// test-directive valid
// test-directive return_code: 1

/* A simple arithmetic test case */

int main(void) {
    unsigned u = 2147483647u;
    return (u + 2u == 2147483649u);
}