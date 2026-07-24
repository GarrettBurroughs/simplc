// test-directive valid
// test-directive return_code: 0
// test-directive include return_pointer_to_array_client.c

// given a nested array of longs, return a pointer to one row in the array
long (*return_row(long (*arr)[3][4], int idx))[4] {
    return arr[idx];
}