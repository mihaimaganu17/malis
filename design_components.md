# Design Components of a language

## Data Types
- Booleans
- Numbers: Int, unsigned, double, etc
- Strings: Which encoding?
- Nil / null

## Expressions
Expressions produce a value or evaluate to a value

- What type? Infix, prefix, postfix, mixfix?
- Arithmentic: Add, sub, mul, div
- Comparison: >, <, ==, !=, >=, <=
- Logical: not, and, or, shift, modulo,
- Precedence and grouping: Can be grouped by () and must chose operator precedence

## Statements
Statements produce an effect and by definition they do not evaluate to a value. Usually modifying
some state, reading input, productin output etc.
Ex: `print "Hello World"`

Usually an expression followed by a semicolon `;` promotes it to a statement.
You can wrap multiple statements in a block with `{}`

## Variables
`var breakfast`
`var trip = "Paris"`

## Control flow
- `if` executes one of 2 statements based on some condition
- `while` executes a block as long as the condition expression evaluates to true
- `for` loops. Initial value, condition and stepping
- Dynamic dispatch -> Which implementation of a polymorphic operation(function) to select at
runtime.

## Functions
- Calling a function: `make_homework(books, pen, internet)`
- Declaring a functions: `fun print_you(name, age) { ... }`

Arg vs Param:
- An `argument` is an actual value you pass to a function when you call it.
    As such, a function call has an argument list
- A `parameter` is a variable that holds the value of the argument inside the body of the function.
    Thus, a function declaration has a parameter list

Function declaration vs definition in C:
- Declaration binds the function's type to its name, such that call can be type-checked
    But does not provide a body
- Definition declares the function and also fills in the body so that the function can be compiled

- `return` statement

## Closures
Functions can be `first class` which means the are real values that you can:
- get a referenc to,
- store in variables,
- pass around, etc

Since function declarations are statements, you can declare local functions inside another function.

```
fun outer_func() {
    fun local_func() {
        print "I'm local!";
    }
}
```

Combining local functions, first-class functions and block scope:

```
fun return_func() {
    var outside = "outside";

    fun inner() {
        print outside;
    }

    return inner;
}

var fn = return_func();
fn();
```

In the example above, `inner()` accesses a local variable declared outside of its body in the
surrounding function.
For that to work, `inner()` has to "hold on" to references to any surrounding variables that it
uses so that they stay around even after the outer function has returned.
These are called `closures` because they `close over` and hold on to the variables it needs.
This adds a bit of complexity, since variable scope does not work strictly like a stack.

## Classes
Advantages:
- Defining compound data types to bundle blobs of stuff together.
- Avoiding prefixing functions with the type's name
    Because methods are scoped to the object

Classes vs Prototypes

1. Classes

Class-based languages define 2 concepts:
- Instances: Store the state for each object and have a referenc to the instance's class.
- Classes: Contain the methods and the inheritance chain

To call a method on an intance, there is always a level of indirection:
    You look up the instance's class and then you find the method there.

Static dispatch:
- Typical in statically typed languaged like C++, method lookup happend at compile time baed on
the static type of the instance
Dynamic dispatch:
- looks up the class of the actual instance object at runtime.
    This is how virtual methods in statically typed languages and all methods in a dynamically typed
    language work.

2. Prototypes
In prototype-based languages there are only `objects`, no classes and each individual object may
contain state and methods.
Object can directly inherit from each other (or "delegate to" in prototypal lingo:)

### Class basics

Returning to classes:
Ex from Lox:
```
class Breakfast {
    cook() {
        print "Egss a-fyin'!";
    }

    server(who) {
        print "Enjoy the breakfats, " + who + ".";
    }
}
```

The body of the class contains its methods.
They look like function declarations but have no keyword.
When a class declaration is executed, you can create a class object and store that in a variable
name after the class.
Classes can be first class.

```
// Store in a variable
var some_var = Breakfast;

// Pass to a function
some_func(Breakfast);
```

Consider the class itself as a factory function for instances. Call a class like a function and
it produces a new instance of itself

```
var breakfast = Breakfast();
print breakfast;
```

### Instantiation and initialization
Classes should encapsulate both:
- Behaviour: Through methods
- State: Through properties/fields

Properties can be freely added onto objects
```
breakfast.meat = "sausage";
breakfast.bread = "sourdough";
```

Assigning to a field creates it if it doesn't already exist. (Seems like a `HashMap`)
Accessing a field or method on the current object from within a method could be done with `this`.

```
class Breakfast {
    serve(who) {
        print "Enjoy you" + this.meat + " and " +
            this.bread + ", " + who + ".";
    }
}
```

Encapsulating data withing an object is ensuring the object is in a valid state when it is created.
For that, we can define an initializer, with a default name like `init()`, which is called
automatically when the object is constructed.
Any parameters passed to the class are forwarded to its initializer.

```
class Breakfast {
    init(meat, bread) {
        this.meat = meat;
        this.bread = bread;
    }
}

var bacon_and_toast = Breakfast("bacon", "toast");
bacon_and_toast.serve("Dear Reader");
```

### Inheritance
Define methods which can be reused across multiple classes and objects.
Inheritance can be:
- Single
- Multiple

```
class Brunch < Breakfast {
    drink() {
        print "How about a Gin Tonic?";
    }
}
```
Every instance of a subclass is an instance of its superclass too
    But there may be instances of the superclass that are not instances of the subclass.
This means that the set of subclass objects is smaller that the superclass's set

In the example above, Brunch is `derived class` or `subclass`
and Breakfast is the `base class` or `superclass`.
Every method defined in the superclass is also available to its subclasses.

```
var benedict = Brunch("ham", "English muffin");
benedict.serve("Noble Reader");
```
The `init()` method also gets inherited.
In practice, the subclass usually wants to define its own `init()` method too. But the original
one also needs to be called so that the superclass can maintain its state.

There is need for a way to call a method on our own instance without hitting our own methods.
We can use `super` for that

```
class Brunc < Breakfast {
    init(meat, bread, drink) {
        super.init(meat, bread);
        this.drink = drink;
    }
}
```

## The Standard Library
Set of functionality that is implemented directly in the interpreter and that all user-defined
behavior is build on top of.
