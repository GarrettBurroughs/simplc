// test-directive valid
// test-directive return_code: 1

int main(void) {
    int x = 0;
    if (0)
        ;
    else
        x = 1;
    return x;
}