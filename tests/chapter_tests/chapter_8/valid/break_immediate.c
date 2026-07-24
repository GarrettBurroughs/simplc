// test-directive valid
// test-directive return_code: 1

int main(void) {
    int a = 10;
    while ((a = 1))
        break;
    return a;
}
