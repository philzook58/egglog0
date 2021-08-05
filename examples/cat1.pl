/* Axioms of category */

/* Typing rules */
/* identity exists */
type(id(A)) = hom(A,A) :- ob = type(A). 
/* ob = type(A) is probabnly slightly more efficient to search for than type(A) = ob */

/* Composition exists if types work out */
type(comp(F,G)) = hom(A,C) :- hom(A,B) = type(F), hom(B,C) = type(G).


/* Now this is possible 
F = comp(id(A), F) :- hom(A,B) = type(F).
F = comp(F, id(B)) :- hom(A,B) = type(F).
*/
/* 
More efficient form 
*/
F <- comp(id(A), F).
F <- comp(F, id(A)).


/* associativity of composition */
comp(comp(F,G),H) <-> comp(F, comp(G,H)).


/* specify types */
type(a) = ob.
type(b) = ob.
type(c) = ob.
type(d) = ob.

type(f) = hom(a,b).
type(g) = hom(b,d).
type(h) = hom(a,c).
type(k) = hom(c,d).

/* assume g is monic */
F = H :- comp(F,g) = comp(H,g), hom(A,b) = type(F), hom(A,b) = type(H). 

/* square is pullback */
comp(f,g) = comp(h,k). /* square commutes */

/* universal triangle 1 */
comp(univ(F,H,E),h) = H
 :- comp(F,g) = comp(H,k), hom(E,b) = type(F), hom(E,c) = type(H).

/* universal triangle 2 */
comp(univ(F,H,E),f) = F 
 :- comp(F,g) = comp(H,k), hom(E,b) = type(F), hom(E,c) = type(H).


/* uniqueness given square and triangles */
U = univ(F,H,E) :- 
    comp(F,g) = comp(H,k), comp(U,h) = H, comp(U,f) = F, hom(E,b) = type(F), hom(E,c) = type(H).

/* Theorem:
h is monic. => forall P Q, comp(P,h) = comp(Q,h), dom(P) = dom(Q) => P = Q

We can take p and q to left of seqeunt, or intro them.
They are arbitrary symbols. We introduce the domain as z.
*/
type(z) = ob.
type(p) = hom(z,a).
type(q) = hom(z,a).
comp(p,h) = comp(q,h).

?- p = q.
?- f = g.
?- p = f.
?- k = h.
?- k = g.
?- type(comp(p,h)) = T.
?- type(id(a)) = hom(a,a).
?- comp(comp(id(a), h), k) = T.
