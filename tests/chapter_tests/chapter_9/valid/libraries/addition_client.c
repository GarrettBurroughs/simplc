// test-directive valid
// test-directive return_code: 3
// test-directive include addition.c

int add(int x, int y);

int main(void) {
    return add(1, 2);
}