// test-directive valid
// test-directive return_code: 0
// test-directive include global_array_client.c

long arr[4] = {1, 2, 3, 4};

int double_each_element(void) {
    for (int i = 0; i < 4; i = i + 1) {
        arr[i] = arr[i] * 2;
    }

    return 0;
}