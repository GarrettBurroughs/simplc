// test-directive valid
// test-directive return_code: 2
// test-directive extra_credit: switch

int main(void) {
    int a = 0;
    switch (a = 1) {
        case 0:
            return 10;
        case 1:
            a = a * 2;
            break;
        default:
            a = 99;
    }
    return a;
}