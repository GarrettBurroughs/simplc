// test-directive valid
// test-directive return_code: 2

int main(void) {
    int a = 0;
    int b = 1;
    if (a)
        b = 1;
    else if (~b)
        b = 2;
    return b;
}