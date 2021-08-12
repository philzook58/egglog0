/*
Defunctionalized SKI Combinators
https://en.wikipedia.org/wiki/SKI_combinator_calculus
*/

apply(i,X) <-> i(X).

apply(k,X) <-> k(X).
apply(k(X),Y) <-> k(X,Y).

apply(s,X) <-> s(X).
apply(s(X),Y) <-> s(X,Y).
apply(s(X,Y),Z) <-> s(X,Y,Z).

X <- i(X).
Y <- k(X,Y).
apply(apply(X,Z),apply(Y,Z)) <- s(X,Y,Z).

k(i(k),i(i)).
?- k(i(k),i(i)) = A.

s(k,k,s).
?- s(k,k,s) = A.