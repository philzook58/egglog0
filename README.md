# egglog

Using the [egg](https://egraphs-good.github.io/) library with a file format and semantics similar to datalog.

Explanatory blog posts: 
- <https://www.philipzucker.com/egglog-checkpoint/>
- <https://www.philipzucker.com/egglog2-monic/>
## Try It Online!!!

<http://www.philipzucker.com/egglog/>

## Building

To run on a file locally:
`cargo run --release tests/examples.pl`

To build the wasm library:
`wasm-pack build --target web`

Note: I started modifying egg a bit. I exposed the Subst datatypes field.

## Ideas


- [x] MultiApplier could be useful / efficient
- [ ] Using Conditional Equals could be useful if all variables known
- [ ] But really getting patterns to compile with substituion pieces considered known subsumes this optimization I think in modern egg with yihong's optimization.
- [ ] Sanity checks that needed variables exist would be good. when it does crash it names rules, so that's something.
- [ ] May want to run Runner multiple times since it may not get restarted. Currently I have that vec![0] hack
- [ ] _ for dummy variables
- [ ] The ability to check to see if something is in the egraph.
- [ ] graphviz dumping the egraph. graphviz wasm?
- [ ] harrop formula
- [ ] merge_subst that doesn't copy?
- [ ] Give rules names. Keep a hash table of them?
- [x] Queries with variables
- [x] Queries should be conjunctions
- [X] a REPL would be sweet. especially if we have higher order rules, we could watch the database, add queries
- [ ] termination based on the query condition
- [ ] side effectful searchers and appliers (printing mostly), functions.
- [ ] Astsize with weighting? Does that get me anywhere?
- [ ] infix operators
- [ ] better printers
- [ ] rewrite/proof files that allow intermediate queriess. set of support?
- [x] cli
- [ ] smtlib subset (forall (a b ) (= (f a) (g c))  ) ! :pattern) or horn cluase style.
- [ ] vaguely ML/coq style synax
- [ ] tptp syntax?
- [ ] push pop directives instead of clear.
- [ ] only allow stuff that compresses the egraph? Appliers that do not add terms to the egraph or only add a couple? Or keeps counts.
- [ ] directives to changes egraph params. or flags?
- [ ] Macros/simplification stage?
- [ ] typed symbollang - would this even be an optimization?
- [ ] defunctionalization of lambdas. lambda-egglog
- [ ] backchain until stumped? depth limitted backchain?
- [ ] hashlog - experiment with same thing but on hashcons instead of egraph. Easier to understand semi naive?
- [ ] epeg extraction
- [ ] faster multipattern via compilation
- [ ] integerate analysis?
- [ ] gensym
- [ ] serialize the egraph
- [ ] negation checks. nonlogical
- [ ] cli options to the runner

### Tests

`cram tests/cram/*.t -i`