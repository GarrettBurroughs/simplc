// test-directive invalid
// test-directive extra_credit: increment

// Can't apply prefix ++/-- to string literal
int main(void) {
    ++"foo";
    return 0;
}