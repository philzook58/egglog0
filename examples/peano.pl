
nat(z).
nat(s(X)) :- nat(X).
nat(plus(X,Y)) :- nat(X), nat(Y).

X <-> plus(z,X). /* Hmm. Is this even right? I need X to be a nat */
plus(s(X),Y) <-> s(plus(X,Y)). 

/* Can I prove these? */
/*
plus(X,Y) <- plus(Y,X).
plus(plus(X,Y),Z) <-> plus(X,plus(Y,Z)).
plus(X,plus(Y,Z)) <- plus(X,Y,Z).
*/

/* These solve probably because we enumerate the integers */
?- plus(s(z),s(s(z))) = Z.
?- plus(Y, s(z)) = s(s(s(s(s(z))))).
nat(c).
?- plus(c,z) = plus(z,c).  /* This will never prove. Needs reasoning by cases. */


/*
binary : 
i(n) = 2*n+1
o(n) = 2*n

2*n + 2 * m  = 2*(m+n)
2*n+1 = i(s(u()))


*/