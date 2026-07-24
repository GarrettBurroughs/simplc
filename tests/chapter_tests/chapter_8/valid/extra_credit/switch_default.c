// test-directive valid
// test-directive return_code: 22
// test-directive extra_credit: switch

int main(void) {
    int a = 0;
    switch(a) {
        case 1:
            return 1;
        case 2:
            return 9;
        case 4:
            a = 11;
            break;
        default:
            a = 22;
    }
    return a;
}