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

## Acknowledgements
It is the Rust version of the Java written Jlox with some syntax additions provided by solving
recommended exercises by the author. You can find more on [crafting interpreters](https://craftinginterpreters.com/)
