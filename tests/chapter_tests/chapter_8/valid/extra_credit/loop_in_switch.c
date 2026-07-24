// test-directive valid
// test-directive return_code: 123
// test-directive extra_credit: switch

int main(void) {
    int cond = 10;
    switch (cond) {
        case 1:
            return 0;
        case 10:
            for (int i = 0; i < 5; i = i + 1) {
                cond = cond - 1;
                if (cond == 8)
                    // make sure this breaks out of loop,
                    // not switch
                    break;
            }
            return 123;
        default:
            return 2;
    }
    return 3;
}