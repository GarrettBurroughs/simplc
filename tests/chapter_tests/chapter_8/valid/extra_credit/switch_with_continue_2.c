// test-directive valid
// test-directive return_code: 5
// test-directive extra_credit: switch

int main(void) {
    int sum = 0;
    for (int i = 0; i < 10; i = i + 1) {
        switch(i % 2) {
            // make sure continue in switch in loop is permitted
            case 0: continue;
            default: sum = sum + 1;
        }
    }
    return sum;
}