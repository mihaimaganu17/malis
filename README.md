# Malis
Malis is currently an interpreted language written in Rust. It can both execute a `.ms` Malis file
and as well act as a REPL in the terminal.

## Instalation
From the root directory run
```
cargo install --path .
```

## Running
You could just run the repl using
```
malis
```
and enter your code there for the interpreter.

or

You could paste the code in a `.ms` file and have malis execute it
```
malis file.ms
```

## Syntax
### Utilities
Malis support a builtin `print` keyword and C-style oneline comments
```
print "Mata mare"; // Prints 'Mata mare'
```

### Variable declaration and types
Variable declaration is done with the keyword `var`. Malis supports dynamic typing, and supports
all the conventional types
```
var specimen = "Human";
var money = 0.123;
var friends = -1;
var am_I_beautiful = true;
```
Reasssigning of variable is also possible as the default (and only) state of each variable is
mutable.
```
specimen = "Humanoid";
money = money + 10;
```

### Operations
Malis supports all the basic calculator operation. Addition, subtraction, multiplication and
division on integers.
```
var gehalt = 1 + 9 + 3;
var bonus = gehalt * 1.10;
var after_bills = bonus - 6;
var split = after_bills / 2;
print split;
```
Addition is also supported on strings.
```
var speed = "300";
print "This car goes " + speed + "!";
```

We also support logic operators for boolean evaluataion
```
print true && false;
```

### Scopes
Malis supports scopes through the use of curly brackets.
```
var outside = -10;
{
    var inside = outside + 20;
}
```
Scoping also supports shadowing variables
```
var outside = -10;
{
    var outside = 20;
    print outside; // Prints '20'
}
```

### Control flow
Control flow covers branching logic, with if-else
```
var sunny = true;
if (sunny)
    print "Merg afara";

var warm = false;
if (sunny && warm) {
    print "Merg afara";
} else {
    print "Nu e nevoie";
}
```

## Acknowledgements
It is the Rust version of the Java written Jlox with some syntax additions provided by solving
recommended exercises by the author. You can find more on [crafting interpreters](https://craftinginterpreters.com/)
