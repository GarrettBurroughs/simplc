// test-directive valid
// test-directive return_code: 21
// test-directive extra_credit: increment

int main(void) {
    int product = 1;
    for (int i = 0; i < 10; i++) {
        product = product + 2;
    }
    return product;
}