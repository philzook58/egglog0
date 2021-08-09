/* Doesn't work. Need to implement infix operators 
A + B <- B + A.
A + (B + C) <-> (A + B) + C.

A * B <- B * A.
A * (B * C) <-> (A * B) * C.
*/
plus(X,Y) <- plus(Y,X).
plus(plus(X,Y),Z) <-> plus(X,plus(Y,Z)).
mul(X,Y) <- mul(Y,X).
mul(mul(X,Y),Z) <-> mul(X, mul(Y,Z)).
/* distributive */
mul(X,plus(Y,Z)) <-> plus(mul(X,Y),mul(X,Z)).
X <- mul(one, X).
X <- plus(zero, X).

plus(one,X) <- succ(X).
two = succ(one).
three = succ(two).
four = succ(three).
five = succ(four).
six = succ(five).

plus(mul(x, three), mul(four,x)).
plus(mul(x, three), mul(two, plus(one, mul(four,x)))).
mul(two,two).
?- mul(two,two) = plus(two,two).
?- mul(two,two) = plus(one, X).
?- mul(two,two) = Z.
?- plus(mul(x, three), mul(two, plus(one, mul(four,x)))) = Z.
