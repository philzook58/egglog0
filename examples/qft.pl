
/* mul(adag(i), adag(j)) <-> mul(adag(i), adag(j))  
How to deal with delta_ij
*/

plus(X,Y) <- plus(Y,X).
plus(plus(X,Y),Z) <-> plus(X,plus(Y,Z)).
/* mul(X,Y) <- mul(Y,X). */
mul(mul(X,Y),Z) <-> mul(X, mul(Y,Z)).

mul(a , ket0 ) = zero. 
O <- dag(dag(O)).
dag(ket0) = bra0.
dag(mul(A,B)) <-> mul(dag(B), dag(A)).


mul(dag(a),a) <-> plus(one, mul(a,dag(a)) ).

/* special multiplication for constants?
This is sort of an analysis built intrinsically into the egraph mechanism.

const(zero).
const(one).
const(  succ(X) ) :- const(X).

 */

one = mul(bra0, ket0).

