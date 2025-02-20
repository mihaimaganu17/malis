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
