// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: increment

int main(void) {
    int a = 2;
    int b = 3 + a++;
    int c = 4 + ++b;
    return (a == 3 && b == 6 && c == 10);
}