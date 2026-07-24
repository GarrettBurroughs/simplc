// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: increment

int main(void) {
    int a = 0;

    // branch not taken; we decrement a, but result is pre-decrement value
    if (a--)
        return 0; // failure
    else if (a--) // we do take this one
        return 1;
    return 0; // failure

}