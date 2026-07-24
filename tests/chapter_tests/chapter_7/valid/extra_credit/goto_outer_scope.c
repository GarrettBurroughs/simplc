// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: goto

int main(void) {
    int a = 10;
    int b = 0;
    if (a) {
        int a = 1;
        b = a;
        goto end;
    }
    a = 9;
end:
    return (a == 10 && b == 1);
}