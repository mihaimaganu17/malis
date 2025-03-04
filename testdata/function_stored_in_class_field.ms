class Box {}

fun not_method(argument) {
    print "called function with " + argument;
}

var box = Box();
box.function = not_method;
box.function("argument");
