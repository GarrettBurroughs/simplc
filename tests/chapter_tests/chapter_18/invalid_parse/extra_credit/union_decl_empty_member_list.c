// test-directive invalid
// test-directive extra_credit: union

// union member list cannot be empty
// (note that GCC/Clang allow this as an extenision)
union s {};

int main(void) {
    return 0;
}