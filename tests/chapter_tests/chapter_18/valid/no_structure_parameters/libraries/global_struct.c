// test-directive valid
// test-directive return_code: 0
// test-directive include global_struct_client.c

/* Basic test that we can access a global struct in multiple translation units
 * */
#include "global_struct.h"

void update_struct(void) {
    global.arr[1] = global.arr[0] * 2;
    global.d = 5.0;
}

void update_outer_struct(void) {
    struct s inner = {0, {-1, -1}, 0};
    global_outer.inner = inner;
}