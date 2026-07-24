// test-directive valid
// test-directive return_code: 2
// test-directive extra_credit: increment

int main(void) {
    int a = 0;
    return (++a ? ++a : 0);
}