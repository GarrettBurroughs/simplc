// test-directive valid
// test-directive return_code: 1
// test-directive include division.c

int f(int a, int b, int c, int d);

int main(void) {
    return f(10, 2, 100, 4);
}