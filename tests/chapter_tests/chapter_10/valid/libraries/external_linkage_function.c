// test-directive valid
// test-directive return_code: 0
// test-directive include external_linkage_function_client.c

/* you can redeclare a function multiple times,
 * but only define it once
 */
extern int sum(int a, int b);

int sum(int i, int j) {
    return i + j;
}

int sum(int x, int y);
