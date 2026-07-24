// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: increment

// Test that we can apply ++ and -- to parenthesized expressions
int main(void) {
    int a = 1;
    int b = 2;
    int c = -++(a);
    int d = !(b)--;
    return (a == 2 && b == 1 && c == -2 && d == 0);
}