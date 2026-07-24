// test-directive valid
// test-directive return_code: 2

int main(void) {
    int x = 4;
    for (int i; (i = x - 2); x = i + 1) {
    }
    return x;
}
