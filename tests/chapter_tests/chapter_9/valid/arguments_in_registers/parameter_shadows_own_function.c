// test-directive valid
// test-directive return_code: 2

int a(int a) {
    return a * 2;
}

int main(void) {
    return a(1);
}