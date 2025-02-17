fun add(a, b) {
    print a + b;
}

fun count(n) {
  if (n > 1) count(n - 1);
  print n;
}

count(20);
add(1, 2);

