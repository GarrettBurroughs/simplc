// test-directive valid
// test-directive return_code: 0

int main(void) {
    for (int i = 400; ; i = i - 100)
        if (i == 100)
            return 0;
}
