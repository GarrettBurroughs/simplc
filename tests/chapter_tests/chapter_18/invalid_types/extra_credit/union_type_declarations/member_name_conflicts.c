// test-directive invalid
// test-directive extra_credit: union

/* You can't declare two members of the same union with the same name */
union u {
    int a;
    int a;
};

int main(void) {
    return 0;
}