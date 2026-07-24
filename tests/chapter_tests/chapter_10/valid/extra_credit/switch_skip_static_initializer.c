// test-directive valid
// test-directive return_code: 10
// test-directive extra_credit: switch

int a = 3;
int main(void) {
    switch (a) {
        case 1:;
            /* Since x is static, it's initialized at program startup,
             * so its value will be 10 even though we jump over this declaration
             */
            static int x = 10;
            // we DON'T execute this, since it's a statement rather than a
            // static initializer
            x = 0;
        case 3:
            return x; // expected return value: 10
    }
    return 0; // fail
}