// test-directive valid
// test-directive return_code: 1
// test-directive include sizeof_extern.c


extern double large_array[1000][2000];

int main(void) {
    return sizeof large_array == 16000000;
}