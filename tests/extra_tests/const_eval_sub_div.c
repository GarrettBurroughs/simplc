// test-directive valid
// test-directive return_code: 0

int main(void) {
    int x = 0;
    switch (3) {
        case 5 - 2:
            x = 0;
            break;
        default:
            x = 1;
            break;
    }
    return x;
}
