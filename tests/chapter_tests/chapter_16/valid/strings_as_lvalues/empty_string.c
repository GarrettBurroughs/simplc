// test-directive valid
// test-directive return_code: 0

/* Test that we add a terminating null byte to the empty string */
int main(void) {
    char *empty = "";
    return empty[0];
}