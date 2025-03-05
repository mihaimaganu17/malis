class Thing {
    get_callback() {
        fun local_function() {
            print self;
        }
        return local_function;
    }
}

var callback = Thing().get_callback();
print callback;
callback();
