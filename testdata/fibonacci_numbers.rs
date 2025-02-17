fun fib(n) {
    print "original =" + n;
    if (n <= 1) return "n=" + n;
    return fib(n-2) + fib(n-1);
}

for (var i = 0; i < 3; i = i + 1) {
    print "i = " + i;
    print fib(i);
}
