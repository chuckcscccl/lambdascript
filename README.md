## Lambdascript

**Lambdascript** executes beta-reduction steps on terms of the lambda calculus,
with the option of inferring polymorphic types before reduction.
It is not a high-performance implementation of lambda
calculus. Rather, the tool serves three primary purposes, all of which
are illustrational or educational in nature:

  1. It demonstrates the usage of the **[rustlr](https://docs.rs/rustlr/latest/rustlr/index.html)** *parser generator*.  The LALR(1) grammar for lambdascript in rustlr format
  is given [here](https://cs.hofstra.edu/~cscccl/rustlr_project/lambdascript/untyped.grammar).

  2. For introductory level students in a programming languages class, the
  tools show every step of beta reduction, including alpha-conversions where
  necessary, in reducing a term to normal form.  It includes both full
  beta-normalization using the normal order (call-by-name) strategy as well
  as weak normalization using call-by-value.  Definitions can be given
  for terms such as S, K, I.

  3. For more advanced students, the source code of the program demonstrates
  how lambda terms can be represented in abstract syntax and how
  reductions can be implemented.  The typing module demontrates the
  how types can be inferred using the unification algorithm.

### Usage
The program was written in Rust and should be installed as an executable: **`cargo install lambdascript`**. You must have Rust installed (from <https://rust-lang.org>) to execute the cargo command.

The program can read from a script or interactively read from stdin. Expressions and defintions are separated by ; (semicolon).  Here's an example of reading and evaluating from stdin, which can be initiated by running the executable.

```
<<< (lambda x.x (lambda y.x y)) y;
(λx.x (λy.x y)) y
 =>  y (λy1.y y1)
```
Lambdascript uses standard syntax for lambda terms: application associates to
the left and application binds tighter than abstraction, meaning that the
scope of a λ extends to the right as far as possible unless bounded by
parentheses.  Lambda expressions inside applications must always by bound
by parentheses: so `x lambda y.y` should be replaced with `x (lambda y.y)`.
There is no notation for types in terms: types are inferred for terms.

Given a file `simple.ls` with the following contents:
```
define I = lambda x.x;
define K = lambda x.lambda y.x;
define lazy INFINITY = (lambda x.x x) (lambda x.x x);

K I INFINITY x;
```
**`lambdascript simple.ls`** produces the following output:
```
K I INFINITY x
= (λxλy.x) I INFINITY x
 =>  (λy.I) INFINITY x
= (λyλx.x) INFINITY x
 =>  (λx.x) x
 =>  x
```
The reduction terminated because normal-order (call-by-name)
evaluation is applied by default.  If the the last line of the file
was replaced with `weak (K I INFINITY x)`, then weak reduction using
call-by-value will take place, resulting in an infinite loop.  There
will likewise be an infinite loop if `lazy` was missing from the
definition of `INFINITY`.  Full, normal-order evaluation and weak
call-by-value are the only reduction strategies implemented in
lambdascript.

Given a file `simpletyped.ls` with the following contents:
```
define I = lambda x.x;
define K = lambda x.lambda y.x;
define S = lambda x.lambda y.lambda z.x z (y z);
define SKI = S K I;
```
**lambdascript typed simpletyped.ls** produces the following output:
```
THE INFERRED TYPE OF I IS  Π(a -> a)
THE INFERRED TYPE OF K IS  Π(a -> b -> a)
THE INFERRED TYPE OF S IS  Π((h -> f -> g) -> (h -> f) -> h -> g)
THE INFERRED TYPE OF SKI IS  Π(r -> r)
```
where `Π` represents quantification over type variables.

All variables and identifiers are limited to a length of 15 characters.

After a script is executed, the interpreter automatically enters interactive
mode with the definitions from the script still available.

The file **[pure.ls](https://cs.hofstra.edu/~cscccl/rustlr_project/lambdascript/pure.ls)** contains a full list of definitions of well-known (untyped) lambda-calculus
combinators.

#### Interactive Interpreter Directives

At the `<<<` prompt the following special directives can be given:

  * `exit` or `quit` : exits the program
  * `typed` : switch to typed mode: types will be inferred and untypable
    terms will not be reduced.
  * `untyped` : switch to untyped mode
  * `use lambda` or `use lam` or `use Lam` or `use \`: On some systems,
    the Greek character λ (unicode 0x03BB) may fail to display properly.
    To change the symbol displayed for lambda, you can choose between one
    of four alternatives (the choices are limited to these four because the
    symbol must be a statically allocated string).
  * `use greek` or `use unicode`: reverts to displaying λ, which is the default
  * `trace off`: turns off the displaying of intermediate reduction steps: only the initial term and the final normal form are shown
  * `trace medium`: Beta-reduction steps are shown, but not the expansion
    of defintions nor alpha-conversion
  * `trace on` or `trace max`: restore displaying of all steps: this is the
    default

-----------------------------

