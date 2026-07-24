// test-directive valid
// test-directive return_code: 3

/* A basic dereferencing test case */

int main(void) {
    int x = 3;
    int *ptr = &x;
    return *ptr;
}