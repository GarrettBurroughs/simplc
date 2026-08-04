// test-directive valid
// test-directive return_code: 25

int foo(int a) {
    switch (a) {
        case 1:
            return 10;
        case 2:
            return 20;
        default:
            return 30;
    }
}

int bar(int b) {
    switch (b) {
        case 1:
            return 5;
        case 2:
            return 15;
        default:
            return 25;
    }
}

int main(void) {
    return foo(1) + bar(2);
}
