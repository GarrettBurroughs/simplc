// test-directive valid
// test-directive return_code: 0
// test-directive extra_credit: compound

int main(void) {
    long l = -34359738368l; // -2^35
    int i = -10;
    /* We should convert i to a long, then subtract from l */
    l -= i;
    if (l != -34359738358l) {
        return 1;
    }
    return 0;
}