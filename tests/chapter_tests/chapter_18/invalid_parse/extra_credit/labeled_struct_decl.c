// test-directive invalid
// test-directive extra_credit: goto

// Because a structure declaration isn't a statement, it can't appear right
// after a label. NOTE: this is valid as of C23
int main(void) {
foo:
    struct s {
        int a;
    };
    return 0;
}