// test-directive valid
// test-directive return_code: 0
// test-directive include long_global_var_client.c

long int l = 8589934592l; // 2^33

long return_l(void) {
    return l;
}

int return_l_as_int(void) {
    return l;
}