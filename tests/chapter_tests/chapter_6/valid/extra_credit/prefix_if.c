// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: increment

int main(void) {
    int a = -1;

    // branch not taken; we increment a and result is 0
    if (++a)
        return 0; // failure
    else if (++a) // we do take this branch
        return 1;
    return 0; // failure

}