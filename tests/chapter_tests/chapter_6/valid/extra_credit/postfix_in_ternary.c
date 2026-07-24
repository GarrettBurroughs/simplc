// test-directive valid
// test-directive return_code: 9
// test-directive extra_credit: increment

int main(void) {
    int x = 10;
    x - 10 ? 0 : x--;
    return x;
}