// test-directive valid
// test-directive return_code: 0
// test-directive stdout: "H"
// test-directive include system_call_client.c

int putchar(int c);

int incr_and_print(int b) {
    return putchar(b + 2);
}