// test-directive invalid
// test-directive extra_credit: compound

int x(void);

int main(void) {
    int a = 3;
    a += x;
    return 0;
}