// test-directive valid
// test-directive return_code: 1
// test-directive include extern_double.c

/* Test linking against a double defined in another file */
extern double d;

int main(void) {
    return d == 1e20;
}