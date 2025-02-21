# Rules
A variable usage refers to the preceding declaration with the same name in the innermost scope that
encloses the expression where the variable is used.

## Sematinc Analysis

Our interpreter resolves a variable—tracks down which declaration it refers to—each and every time
the variable expression is evaluated. If that variable is swaddled inside a loop that runs a
thousand times, that variable gets re-resolved a thousand times.

We know static scope means that a variable usage always resolves to the same declaration, which can
be determined just by looking at the text. Given that, why are we doing it dynamically every time?
Doing so doesn’t just open the hole that leads to our annoying bug, it’s also needlessly slow.

A better solution is to resolve each variable use once. Write a chunk of code that inspects the
user’s program, finds every variable mentioned, and figures out which declaration each refers to.
This process is an example of a semantic analysis. Where a parser tells only if a program is
grammatically correct (a syntactic analysis), semantic analysis goes farther and starts to figure
out what pieces of the program actually mean. In this case, our analysis will resolve variable
bindings. We’ll know not just that an expression is a variable, but which variable it is

## Resolver for variable resolution

After the parser produces the syntax tree, but before the interpreter starts executing it, we’ll do
a single walk over the tree to resolve all of the variables it contains. Additional passes between
parsing and execution are common. If Lox had static types, we could slide a type checker in there.
Optimizations are often implemented in separate passes like this too. Basically, any work that
doesn’t rely on state that’s only available at runtime can be done in this way.

Our variable resolution pass works like a sort of mini-interpreter. It walks the tree, visiting
each node, but a static analysis is different from a dynamic execution:

There are no side effects. When the static analysis visits a print statement, it doesn’t actually
print anything. Calls to native functions or other operations that reach out to the outside world
are stubbed out and have no effect.

There is no control flow. Loops are visited only once. Both branches are visited in if statements.
Logic operators are not short-circuited.

## Resolving variable declarations

We split binding into two steps, declaring then defining, in order to handle funny edge cases like
this:

```
var a = "outer";
{
  var a = a;
}
```

What happens when the initializer for a local variable refers to a variable with the same name as
the variable being declared? We have a few options:

1. Run the initializer, then put the new variable in scope. Here, the new local a would be
initialized with “outer”, the value of the global one. In other words, the previous declaration
would desugar to:

```
var temp = a; // Run the initializer.
var a;        // Declare the variable.
a = temp;     // Initialize it.
```

2. Put the new variable in scope, then run the initializer. This means you could observe a variable
before it’s initialized, so we would need to figure out what value it would have then. Probably nil.
That means the new local a would be re-initialized to its own implicitly initialized value, nil.
Now the desugaring would look like:

```
var a; // Define the variable.
a = a; // Run the initializer.
```

3. Make it an error to reference a variable in its initializer. Have the interpreter fail either at
compile time or runtime if an initializer mentions the variable being initialized.

Do either of those first two options look like something a user actually wants? Shadowing is rare
and often an error, so initializing a shadowing variable based on the value of the shadowed one
seems unlikely to be deliberate.

The second option is even less useful. The new variable will always have the value nil. There is
never any point in mentioning it by name. You could use an explicit nil instead.
