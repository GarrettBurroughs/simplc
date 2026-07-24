// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: increment

int main(void) {
    int i = 100;
    int count = 0;
    while (i--) count++;
    if (count != 100)
        return 0;
    i = 100;
    count = 0;
    while (--i) count++;
    if (count != 99)
        return 0;
    return 1;
}