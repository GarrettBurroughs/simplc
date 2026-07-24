// test-directive valid
// test-directive return_code: 1

int main(void) {
    int a = -2593;
    a = a % 3;
    int b = -a;
    return b;
}