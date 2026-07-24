// test-directive valid
// test-directive return_code: 76
// test-directive extra_credit: bitwise

/* test support for left and right shift operations where the right operand
 * (i.e. the shift count) is the result of another expression, not a constant.
 */

int main(void) {
    return (4 << (2 * 2)) + (100 >> (1 + 2)); // 64 + 12 = 76
}