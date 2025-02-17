fun add(a, b) {
    print a + b;
}

fun count(n) {
  if (n > 1) add(n - 1, 0);
  print n;
}

print count(3);

