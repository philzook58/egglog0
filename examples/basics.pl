/* Base equality facts */
y = x.
z = y.
/* Base egraph insertions. No implied equalities. For "seeding" the egraph. */
f(x).
bar(boo).
plus(p,r).

/* general looking prolog-like rules */
bar(X) :- f(X) = q.
biz(Z) = baz(biz(boz)) :- fizzy(floozy), buppo(Z).

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
?- f(x) = x.
?- x = x.
?- y = x.
?- plus(p,r) = plus(r,p).
?- junk(boo) = otherjunk(baz).

/* Query simplification */
f(f(f(f(x)))).
?-  f(f(f(f(x)))) = X.

/*
TODO: Directives.
:- node_limit(1000).
:- include("mylibrary.pl")
*/