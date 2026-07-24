// test-directive valid
// test-directive return_code: 0
// test-directive include external_tentative_var_client.c

/* Tentatively define a variable, and make sure it's initialized to 0. */
int x;

int read_x(void) {
    return x;
}