// test-directive valid
// test-directive return_code: 4

int main(void) {
    int a = 1;
    int b = 0;
    a = 3 * (b = a);
    return a + b;
}