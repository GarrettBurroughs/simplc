// test-directive valid
// test-directive return_code: 2

int main(void) {
    int a = 1;
    a != 2 ? a = 2 : 0;
    return a;
}