// test-directive valid
// test-directive return_code: 5
// test-directive extra_credit: goto

// a static variable and label within the same function can share a name
// (make sure we don't e.g. use the naming scheme "main.x" in both cases)
int main(void) {
    static int x = 5;
    goto x;
    x = 0;
x:
    return x; // return 5
}