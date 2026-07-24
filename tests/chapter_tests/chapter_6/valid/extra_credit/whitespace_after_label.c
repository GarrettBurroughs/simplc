// test-directive valid
// test-directive return_code: 1
// test-directive extra_credit: goto

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wunused-label"
#endif
int main(void) {
    goto label2;
    return 0;
    // okay to have space or newline between label and colon
    label1 :
    label2
    :
    return 1;
}