// test-directive valid
// test-directive return_code: 0
// test-directive include internal_hides_external_linkage_client.c

int x = 10;

int read_x(void){
    return x;
}