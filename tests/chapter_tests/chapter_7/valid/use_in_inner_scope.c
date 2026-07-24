// test-directive valid
// test-directive return_code: 3

int main(void)
{
    int x;
    {
        x = 3;
    }
    {
        return x;
    }
}