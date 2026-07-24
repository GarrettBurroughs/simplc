// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: switch

int main(void) {
    int a = 1;
    switch(a) default: return 1;
    return 0;
}