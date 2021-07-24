# egg-dl

Using the egg library with a file format and semantics similar to datalog rules.


`wasm-pack build --target web`

`cargo run --release tests/examples.pl`

### The low down

Egraphs are a topic that has fascinated me for a while. Nearly a year now, which is a bit disturbing. Where is all the time going?
The egraph is a data structure for storing equalities between terms. I've changed what is my favorite method of description of it.
A description I kind of like right now is to consider a naive representation of destructionless rewrite rule application and consider the kind of sharing that is possible. Classical rewriting mutates the trees you're rewriting. Instead of doing that, you could copy the entire tree. This is very wasteful however. If your rewrite occurs near the top of the tree, you can share subtrees via pointers. If you're rewrite happens near the bottom you can share the parent structure by adding the indirection of "eclass" nodes.

Another aspect that has been fascinating me is the analogy to datalog. See here for more about that
[Encoding E-graphs to Souffle Datalog](https://www.philipzucker.com/egraph-datalog/)
Datalog isn't the only system that just collects new inferences, but it is a prominent one. Resolution based theorem proving also has this flavor.
Datalog and the egraph are monotonic, in that they just keep gaining new inferences and never make missteps. Theorem proving like in a DPLL SAT solver or prolog makes guesses that may end up being wrong and needs to backtrack them.
This analogy has had me thinking that something prolog-esque might be a nice surface syntax for describing rules to the egraph system. And I think this makes sense.
There tend to be similarities between rewrite rule systems and prolog for some reason. Recently at work I was decribing how one can encode reachability problems of a control flow graph (CFG) into horn clauses, which is the basis I believe of the usage of constrained horn clause solvers (CHC) in program verification. Cody piped in that he liked to think about it in some way as a rewrite rule system, but that is the church he worships at.

The egraph can itself be considered a kind of database. An equality in the egraph is obviously information, but even just a term being in the egraph can sometimes be usefully interpreted as saying that the term is well formed or well typed. Similar to datalog we can take the bottom up execution semantics.

- Things to the right of `:-` are things to lookup in the database
- Thing to the left of `:-` are things to insert into the database

There is no fundamental obstruction to allowing multiple predicates on the right hand side. This can correspond to both the notion of guarded ematching, but also to the notion of multi triggers that you can find in Z3.

An ematching rewrite rule , for example `plus(A,B) <- plus(B,A)`, in this language can be encoded as
`plus(A,B) = C :- plus(B,A) = C`
This is awkward, and the case will be very common, so it is reasonable to introduce the notation `plus(A,B) <- plus(B,A)` as a syntactic shorthand for this form. In principle the equality you insert on the left side of the clause may have very little to do with what you searched for in the right hand side, other than you need to instantiate every variable in the left hand side from the right hand side.



https://www.philipzucker.com/egraph-datalog/
https://www.philipzucker.com/staging-patterns/
https://www.philipzucker.com/a-simplified-egraph/
https://www.philipzucker.com/union-find-dict/
https://www.philipzucker.com/rust-category/
https://www.philipzucker.com/metatheory-progress/


A Simple, Probably-Not-Exp-Time Disjoint Set in Coq
Partial Evaluation of a Pattern Matcher for E-graphs
A Simplified E-graph Implementation
Union Find Dicts: Dictionaries Keyed on Equivalence Classes
Rewriting Monoidal Categories in the Browser with Egg
Progress on Automated Reasoning for Catlab with Metatheory.jl Egraphs

E-Graph Pattern Matching (Part II)
E-graphs in Julia (Part I)


## Parsing with Nom

[nom](https://github.com/Geal/nom) is a nice parser comnbinator library for rust.
I actually found a seperately packaged prolog parser <https://docs.rs/prolog_parser/0.8.68/prolog_parser/> desigend for [Scryer prolog](https://github.com/mthom/scryer-prolog), but it seemed complicated.

list of combinators https://github.com/Geal/nom/blob/master/doc/choosing_a_combinator.md 

## Bits and Bobbles

- Can we have Harrop formula like in lambda prolog? Forall is a way of introducing gensyms and is very natural for expressing some problems. In principle I can use ematching to increase the rule set, but I bet it would be fairly inefficient, so it would be better to preprocess and optimize. I'd also need a way to check rules for duplicates.
- My parser is trash. I need better errors and I need to deal with whitespace better.