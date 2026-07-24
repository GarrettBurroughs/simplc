// test-directive valid
// test-directive return_code: 1

int main(void) {
    int x = 10;
    int y = 0;
    y = (x = 5) ? x : 2;
    return (x == 5 && y == 5);
}