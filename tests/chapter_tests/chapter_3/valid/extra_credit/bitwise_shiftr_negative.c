// test-directive valid
// test-directive return_code: 255
// test-directive extra_credit: bitwise

/* Make sure we use arithmetic rather than logical right shift.
 * NOTE: right bitshift of negative value is implementation-defined;
 * we follow GCC and use sign extension
 * (see https://gcc.gnu.org/onlinedocs/gcc/Integers-implementation.html)
 * */
int main(void) {
    return -5 >> 30;
}