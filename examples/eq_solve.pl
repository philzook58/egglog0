

/* These are not rewrite rules. They truly need the prolog like capabilities.
Egglog enahnced with floats might be cool. Getting into metatheory territory then.
 */
A = div(C,B) :- mul(A,B) = C. /* guard on nonzero? x - x is zero. */
mul(A,B) = C :- A = div(C,B).
sub(C,B) = A :- plus(A,B) = C.
plus(A,B) = C :- sub(C,B) = A.



/* Properties of div and sub */
mul(X, div(N,D)) <-> div(mul(X,N),D).


/* 
Didn't I have some story about grobner solving? 
Or really like some heurstic waay to reduce to -1 
*/
/* The egg math examples */

/* 
summing geometric series 
1 + x + ... =
S - xS =  1

S(x,zero) = 1
S(x,n) = pow(x,n) + S(x, n - 1).


*/