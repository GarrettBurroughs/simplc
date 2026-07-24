// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: increment

int main(void) {
    int a = 1;
    int b = 2;
    int c = a++;
    int d = b--;
    return (a == 2 && b == 1 && c == 1 && d == 2);
}