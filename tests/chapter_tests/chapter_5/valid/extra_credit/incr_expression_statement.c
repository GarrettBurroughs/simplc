// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: increment

int main(void) {
    int a = 0;
    int b = 0;
    a++;
    ++a;
    ++a;
    b--;
    --b;
    return (a == 3 && b == -2);
}