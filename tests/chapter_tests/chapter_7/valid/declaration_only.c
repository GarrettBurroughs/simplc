// test-directive valid
// test-directive return_code: 1

#ifdef SUPPRESS_WARNINGS
#pragma GCC diagnostic ignored "-Wunused-variable"
#endif
int main(void) {
    int a;
    {
        int b = a = 1;
    }
    return a;
}