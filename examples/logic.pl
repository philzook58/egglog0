
/* https://www.cs.cornell.edu/gries/TechReports/94-1455.pdf 
http://www.mathmeth.com/read.shtml
*/
/* reflection 
The first reflection achieves the substitution property of the equality predicate
The second is just very useful, but also makes the ergaph explode.
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
/* derived? Not in case eq(A,B) = false. */
eq(eq(A,B),C) <-> eq(A,eq(B,C)).
eq(A,B) <- eq(B,A).
not(eq(A,B)) <-> eq(not(A),B).
/* eq(A,A) = true  :- A. already implied by reflection */
eq(false, not(true)).
eq(false, true) = false.
eq(true, false) = false.

false = not(true).
eq(not(eq(p,q)), not(p)).
?- eq(not(eq(p,q)), not(p)) = q.
/* ?- A. */

/* ?- eq(eq(A,B),C) = eq(A,eq(B,C)).
?- eq(A,B) = eq(B,A).
?- eq(A,A) = A.
?- eq(false, not(true)).
?- eq(false,true) = false.
?- false = true.
?- eq(p,q). */

r = true. 
or(p,q).
?- or(r,q) = true. /* How does it know this. Color me actually kind of impressed. */
?- false = true. /* sanity check */

true <- or(P, not(P)).
or(A,or(B,C)) <-> or(or(A,B),C).
or(A,B) <- or(B,A).
A <- or(A,A).
/* Hmmm. Does this type check to the left. You can only `or` booleans */
or(A, eq(B,C)) <-> eq( or(A,B), or(A,C) ).
/* what. This did not crash */
or(P, not(P)) = true.

/* redundant */
or(false,false).
?- or(false,false) = false.
?- or(true,false) = true.
?- or(true,true) = true.
?- or(false,true) = true.

eq(or(P,Q),Q) <-> eq(and(P,Q),P).

?- and(true,false) = false.
?- and(true,true) = true.
?- and(false,false) = false.

and(q,p).

?- and(p,q) = and(q,p).
/*
and(A,B) = true :- true = A, true = B.
*/

imp(P,Q) <-> eq(or(P,Q), Q).

imp(p,p).
?- imp(p,p) = true.
and(p,or(p,q)).
?- and(p,or(p,q)) = p.
imp(p, imp(q, p)).
?- imp(p, imp(q, p)) = true.

imp(imp(s, imp(p,q)),imp(imp(s,p),imp(s,q))).
?- imp(imp(s, imp(p,q)),imp(imp(s,p),imp(s,q))) = true.

?- or(eq(p,true),eq(p,false)) = true.
or(eq(p,red),eq(p,blue)) = true.
?- eq(p,or(red,blue)). /* Why not? */

/*
dijstra's brakcet.
bracket(eq(a,b)) = true.
bracket().
*/