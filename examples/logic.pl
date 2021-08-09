
/* https://www.cs.cornell.edu/gries/TechReports/94-1455.pdf */
/* reflection 
predicate form - in egraph means true. Doesn't make much sense though?
A = B :- eq(A,B).
eq(A,B):- A = B.
*/
A = B :- eq(A,B) = true.
eq(A,B) = true :- A = B.
/* 
This is the rule that's more dangerous.
eq(A,B) = true :- A = B.
*/
/* derived? Not in case eq(A,B) = false.
eq(eq(A,B),C) <-> eq(A,eq(B,C)).
eq(A,B) <- eq(B,A).
eq(A,A) :- A.
eq(false, not(true)).
eq(false, true) = false.
eq(true, false) = false.
*/
false = not(true).



/* ?- eq(eq(A,B),C) = eq(A,eq(B,C)).
?- eq(A,B) = eq(B,A).
?- eq(A,A) = A.
?- eq(false, not(true)).
?- eq(false,true) = false.
?- false = true.
?- eq(p,q). */

p = false.
or(p,q).
?- or(p,q) = true. /* How does it know this. Color me actually kind of impressed. */
?- false = true. /* sanity check */

true <- or(P, not(P)).
or(A,or(B,C)) <-> or(or(A,B),C).
or(A,B) <- or(B,A).
A <- or(A,A).
or(A, eq(B,C)) <-> eq( or(A,B), or(A,C) ).

or(false,false) = false.
or(true,false) = true.
or(true,true) = true.
or(false,true) = true.

/*
and(A,B) = true :- true = A, true = B.

*/