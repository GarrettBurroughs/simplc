// test-directive invalid
// test-directive extra_credit: union

struct s {
    int y;
};

int main(void) {
    // can't parenthesize union tag
    union(s) var;

    return 0;
}