type(id(A)) = hom(A,A) :- ob = type(A). 
/* ob = type(A) is probabnly slightly more efficient to search for than type(A) = ob */
F <- comp(id(A), F).
F <- comp(F, id(A)).

comp(F,id(B)) = F :- type(F) = hom(A,B).
comp(id(A),F) = F :- type(F) = hom(A,B).
/* associativity of composition */
comp(comp(F,G),H) <-> comp(F, comp(G,H)).

/* Composition exists if types work out */
type(comp(F,G)) = hom(A,C) :- hom(A,B) = type(F), hom(B,C) = type(G).

/* A supposed second identity for object a */

type(a) = ob.
type(id2(a)) = hom(a,a).

F <- comp(F,id2(a)).
F <- comp(id2(a),F).
comp(F,id2(a)) = F :- type(F) = hom(A,a).
comp(id2(a),F) = F :- type(F) = hom(a,B).


?- id2(a) = id(a).

/* sanity check. f should not be the identity a priori */
type(f) = hom(a,a).
?- f = id2(a).
