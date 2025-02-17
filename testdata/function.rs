fun add(a, b) {
    print a + b;
}

fun count(n) {
  if (n > 1) count(n - 1);
  print n;
}

fun say_hi(first, last) {
    print "Hi, " + first + " " + last + "!";
}

count(20);
add(1, 2);
say_hi("Dear", "Reader");
