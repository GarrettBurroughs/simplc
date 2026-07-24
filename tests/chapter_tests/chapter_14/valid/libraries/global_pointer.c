// test-directive valid
// test-directive return_code: 1
// test-directive include global_pointer_client.c

double *d_ptr;

int update_thru_ptr(double new_val) {
    *d_ptr = new_val;
    return 0;
}