// test-directive valid
// test-directive return_code: 3

int main(void) {
    int a = 3;
    {
        int a = a = 4;
    }
    return a;
}