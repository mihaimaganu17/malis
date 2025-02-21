var a = "outer";
{
    var a;
    // This should fail as we only declared a variable `a`, which shadows the outer variable `a`
    // but we did not initialize it yet
    print a;
}
