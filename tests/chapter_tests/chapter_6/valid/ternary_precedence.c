// test-directive valid
// test-directive return_code: 20

int main(void) {
    int a = 10;
    // test that || is higher precedence than ?
    return a || 0 ? 20 : 0;
}