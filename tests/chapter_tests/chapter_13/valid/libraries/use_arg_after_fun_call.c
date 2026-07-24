// test-directive valid
// test-directive return_code: 4
// test-directive include use_arg_after_fun_call_client.c

double fun(double x) {
    if (x > 2)
        return x;
    else {
        double ret = fun(x + 2); // ret = 3.0
        return ret + x; // return 4.0
    }
}
