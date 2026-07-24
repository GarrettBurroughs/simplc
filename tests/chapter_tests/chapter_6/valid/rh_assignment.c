// test-directive valid
// test-directive return_code: 1

int main(void) {
    int flag = 1;
    int a = 0;
    flag ? a = 1 : (a = 0);
    return a;
}