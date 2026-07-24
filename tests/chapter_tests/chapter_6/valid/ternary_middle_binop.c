// test-directive valid
// test-directive return_code: 1

int main(void) {
    int a = 1 ? 3 % 2 : 4;
    return a;
}