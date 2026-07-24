// test-directive valid
// test-directive return_code: 16

int main(void) {
    int a = 12345;
    int i;

    for (i = 5; i >= 0; i = i - 1)
        a = a / 3;

    return a;
}
