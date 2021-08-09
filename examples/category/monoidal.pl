 :- include(examples/category/category.pl).
/*  We should make this better */

/* Monoidal. This just sends it off into the stratosphere.
type(otimes(A,B)) = ob :- ob = type(A), ob = type(B).
type(otimes(F,G)) = hom(otimes(A,B),otimes(C,D)) :- hom(A,C) = type(F), hom(B,D) = type(G).
*/

/* The non-generative form */
hom(otimes(A,B), otimes(C,D)) = T :- type(otimes(F,G)) = T, hom(A,C) = type(F), hom(B,D) = type(G).

/* Covers the object case too */
otimes(otimes(F,G),H) <-> otimes(F, otimes(G,H)).

type(munit) = ob.
A <- otimes(munit, A).
A <- otimes(A,munit).
F <- otimes(id(munit), F).
F <- otimes(F, id(munit)).

/* Is this one necessary? */
id(otimes(A,B)) <-> otimes(id(A), id(B)).

comp(otimes(F,G), otimes(P,Q)) <- otimes(comp(F,P), comp(G,Q)).
/* And the other way */

/* Convenience. Macros? */
otimes(A,otimes(B,C)) <- otimes(A,B,C).
otimes(A,otimes(B,otimes(C,D))) <- otimes(A,B,C,D).