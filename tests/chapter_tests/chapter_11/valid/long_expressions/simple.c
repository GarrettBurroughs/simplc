// test-directive valid
// test-directive return_code: 1

/* A simple arithmetic test case */

int main(void) {
    long l = 9223372036854775807l;
    return (l - 2l == 9223372036854775805l);
}