// test-directive invalid
// test-directive extra_credit: switch, goto

int main(void) {
    goto 3;
    switch (3) {
        case 3: return 0;
    }
}