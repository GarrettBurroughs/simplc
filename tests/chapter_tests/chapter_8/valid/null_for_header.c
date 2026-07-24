// test-directive valid
// test-directive return_code: 4

int main(void) {
    int a = 0;
    for (; ; ) {
        a = a + 1;
        if (a > 3)
            break;
    }

    return a;
}
