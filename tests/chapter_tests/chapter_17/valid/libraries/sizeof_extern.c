// test-directive valid
// test-directive return_code: 1
// test-directive include sizeof_extern_client.c

/* Test that we correctly calculate the size of objects declared in other translation units */

double large_array[1000][2000];