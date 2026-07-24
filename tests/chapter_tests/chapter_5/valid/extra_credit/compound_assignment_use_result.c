// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: compound

int main(void) {
    int x = 1;
    int y = x += 3;
    return (x == 4 && y == 4);
}