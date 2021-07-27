---
title: "Egglog"
---

## Try It Out!

<script type="module">
        export { run };
        import init, { run_wasm } from './pkg/egglog.js';

        async function run() {
            await init();
            var query = document.getElementById("query").value;
            let example = `
                f(x) = x.
                /*
                g(X)=f(x):-z.
                f(X) = g(Q) :- Q = X, f(x).
                */
                y = x.
                plus(X,Y) <- plus(Y,X). 
                plus(b,q).
                ?- f(x) = x, x = x, y = x, plus(b,q) = plus(q,b), f(f(x)).
                `
            const result = run_wasm(query);
            console.log(result);
            document.getElementById("result").value = result;

        }
        window.run = run;
        //run();
</script>

<textarea id="query" rows="20" style="width:100%">/* Base equality facts */
y = x.
z = y.
/* Base egraph insertions. No implied equalities. For "seeding" the egraph. */
f(x).
bar(boo).
plus(p,r).

/*
TODO: general looking prolog-like rules 
bar(X) :- f(X) = q.
biz(Z) = baz(biz(boz)) :- fizzy(floozy), buppo(Z).
*/

/* rewrite rules. Variables denoted by capitalization */
plus(X,Y) <- plus(Y,X).
/* In principle syntactic shorthand for plus(X,Y) = C :- plus(Y,X) = C. */
X <- f(f(f(X))).
X <- f(f(X)).

/* bidirectional rewrite. A useful syntactic shorthand for two rewrite rules. */ 
plus(X,plus(Y,Z)) <-> plus(plus(X,Y),Z).

/* Guarded rewrite. */
fiz(baz) <- bar(boo), x = z.


/* Query equalities. Ground queries (no variables) only at the moment.
Note that this does NOT insert into the egraph. Should I change that? Or give a new notation for "insert all subterms and then query"?
 */
?- f(x) = x, x = x, y = x, plus(p,r) = plus(r,p), junk(boo) = otherjunk(baz).

/* Query simplification */
f(f(f(f(x)))).
?-  f(f(f(f(x)))).

/*
TODO: Directives.
:- node_limit(1000).
:- include("mylibrary.pl")
*/
</textarea>
<button onclick="run()">Run</button>
<textarea id="result" rows="20" style="width:100%"> </textarea>

# What is this?

A prolog like syntax for interfacing with the egg egraph library.

Github repo: <https://github.com/philzook58/egglog>
Read more here: <https://www.philipzucker.com/egglog-checkpoint/>

