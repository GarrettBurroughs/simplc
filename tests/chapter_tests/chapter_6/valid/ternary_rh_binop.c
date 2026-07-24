// test-directive valid
// test-directive return_code: 1

#ifdef SUPPRESS_WARNINGS
#ifdef __clang__
#pragma clang diagnostic ignored "-Wconstant-logical-operand"
#endif
#endif
int main(void) {
    return 0 ? 1 : 0 || 2;
}