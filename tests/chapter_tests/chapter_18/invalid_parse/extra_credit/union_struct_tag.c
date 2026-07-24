// test-directive invalid
// test-directive extra_credit: union

// can't use 'struct' as the keyword for a union declaration
union struct {
    int a;
};

int main(void) {
    return 0;
}