// test-directive valid
// test-directive return_code: 10
// test-directive extra_credit: goto

// if goto jumps past the controlling condition in a loop,
// the condition isn't evaluated

int main(void) {
    int i = 1;
    do {
    while_start:
        i = i + 1;
        if (i < 10)
            goto while_start;

    } while (0);
    return i;
}