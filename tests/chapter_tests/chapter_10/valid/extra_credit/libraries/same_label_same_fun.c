// test-directive valid
// test-directive return_code: 0
// test-directive include same_label_same_fun_client.c
// test-directive extra_credit: goto

static int f(void) {
    goto x;
    return 0;
    x:
    return 2;
}

int f_caller(void) {
    return f();
}