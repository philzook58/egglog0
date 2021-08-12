list(nil).
list(cons(X,Y)) :- list(Y), cons(X,Y).

A <- car(cons(A,L)).
L <- cdr(cons(A,L)).

memb(A,cons(A,L)) :- cons(A,L).

/*
How should this work? Aliasing issues. We may discover K1 is K later. 
lookup() <- lookup( K1, cons(kv(K,V),L) ), K1 != K . 
V <- lookup( K, cons(kv(K,V),L) ).
*/

append(nil,Y) <-> Y.
append(Y,nil) <-> Y.
append(cons(X,Y),Z) <-> cons(X, append(Y,Z)).



rev(nil) = nil.
X <- rev(rev(X)).
rev(append(X,Y)) <-> append(rev(Y),rev(X)).


cons(x,cons(y,cons(z,nil))) = l.
?- append(X,Y) = l.

nil <- map(F,nil).
cons(apply(F,X), map(F,L)) <- map(F,cons(X,L)).

apply(F,apply(G,X)) <-> apply(comp(F,G), X).
map(F,map(G,L)) <-> map(comp(F,G),L).

nil <- filter(F,nil).
cons(X,filter(F,L)) <- filter( F, cons(X,L) ), apply(F,X) = true.
filter(F,L) <- filter( F, cons(X,L) ), apply(F,X) = false.

/* defunctionalization */
X <- apply(id, X).
/* What is id? a partial function over the union of all types? A relation? */

filter(id, cons(true,cons(false,nil))).
?- filter(id, cons(true,cons(false,nil))) = Res.
