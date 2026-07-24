// test-directive valid
// test-directive return_code: 10
// test-directive extra_credit: goto

int main(void) {
    goto end;
    /* Since x is static, it's initialized at program startup,
     * so its value will be 10 even though we jump over this declaration
     */
    static int x = 10;
    end:
        return x;
}