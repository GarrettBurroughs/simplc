// test-directive valid
// test-directive return_code: 0

int three(void) {
    return 3;
}

int main(void) {
    /* The function call operator () is higher precedence
     * than unary prefix operators
     */
    return !three();
}