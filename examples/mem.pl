
/* inequality predicate neq */
/* symmettric */
neq(Y,X) :- neq(X,Y).

/* blow up the world if something equal is said to be not equal. */
false :- neq(X,X).

/* injective functions */
neq(neg(X), neg(Y)) :- neq(X,Y).
neq(add(Z,X), add(Z,Y)) :- neq(X,Y), add(Z,X).
neq(mul(Z,X), mul(Z,Y)) :- neq(X,Y), mul(Z,X), mul(Z,Y), notzero(Z).


/* 
Smtlib theory of arrays
https://smtlib.cs.uiowa.edu/theories-ArraysEx.shtml
http://smtlib.cs.uiowa.edu/version1/theories/Arrays.smt
 */


/* select grabs stored value */
V <- select(A ,store(A, V, Mem)).
/* select ignores diffierent addresses */
select(A1,Mem) = E :- select(A1,store(A2,V,Mem)) = E, neq(A1,A2).
/* non aliasing writes commute */
store(A2,V2, store(A1,V1,Mem)) = E :- store(A1,V1, store(A2,V2,Mem)) = E, neq(A1,A2).
/* Aliasing Writes destroy old value. */
store(A, V1, Mem) <- store(A, V1, store(A,V2,Mem)).


zero <- select(A,emp).


/*
Extensionality axioms
(forall i, select(i, Mem) = select(i, Mem2)) -> Mem = Mem2.

Drinker's address.

neq(Mem1,Mem2) -> neq(select(diff(Mem1, Mem2), Mem1), select(diff(Mem1,Mem2), Mem2)



*/


/* Simple properties of arithmetic */
add(X,Y) <- add(Y,X).
add(add(X,Y),Z) <-> add(X,add(Y,Z)).
mul(X,Y) <- mul(Y,X).
mul(mul(X,Y),Z) <-> mul(X, mul(Y,Z)).
/* distributive */
mul(X,add(Y,Z)) <-> add(mul(X,Y),mul(X,Z)).
X <- mul(one, X).
X <- add(zero, X).

/* start with registers being different */
neq(r1,r2).
neq(r2,r3).
neq(r1,r3).

/* initialize egraph with appropiate term */
select(r1, store(r1, a, store(r0, b, mem0))).
?- select(r1, store(r1, a, store(r0, b, mem0))) = E.

select(r3, store(r1, a, store(r1, b, mem0))).
?- select(r3, store(r1, a, store(r1, b, mem0))) = E.


select(add(r1,r2), store(add(zero,add(r2,r1)), a, store(r0, b, mem0))).
?- select(add(r1,r2), store(add(zero,add(r2,r1)), a, store(r0, b, mem0))) = E.

select(add(r0,r3), store(add(r1,r0), a, store(add(r0,r1), b, mem0))) .
?- select(add(r0,r3), store(add(r1,r0), a, store(add(r0,r1), b, mem0))) = E.

/*

bitvectors
https://smtlib.cs.uiowa.edu/theories-FixedSizeBitVectors.shtml

extract and concat will require integer reasoning. Or tagging of all bitvectors by size?
Or len(A) = N?
Or do bitvectors as lists?

A = E :- extract(0, N,concat(A,B)) = E, len(A) = N.
:- extract(0,N,A), len(A) = N.

A <- extract( concat( A,B)  )
zero <- extract(nothing, A)
A <- concat(zero,A).
B <- concat(B,zero).

zero is zro length bitvector


*/

/* ?- neq(R1, R2). */
