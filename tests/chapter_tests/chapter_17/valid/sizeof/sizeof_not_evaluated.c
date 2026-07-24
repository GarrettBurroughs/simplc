// test-directive valid
// test-directive return_code: 4

void exit(int status);
int foo(void) { exit(10); }

int main(void) {
  // make sure foo isn't actually called
  return sizeof(foo());
}