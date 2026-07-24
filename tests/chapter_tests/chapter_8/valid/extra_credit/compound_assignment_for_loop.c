// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: compound

int main(void) {
    int i = 1;
    for (i *= -1; i >= -100; i -=3)
        ;
    return (i == -103);
}