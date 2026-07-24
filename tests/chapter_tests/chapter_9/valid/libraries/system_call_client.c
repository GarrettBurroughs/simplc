// test-directive valid
// test-directive return_code: 0
// test-directive stdout: "H"
// test-directive include system_call.c

int incr_and_print(int c);

int main(void) {
    incr_and_print(70);
    return 0;
}