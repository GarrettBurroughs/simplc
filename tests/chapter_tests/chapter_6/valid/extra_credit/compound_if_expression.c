// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: compound

int main(void) {
    int a = 0;
    if (a += 1)
        return a;
    return 10;
}