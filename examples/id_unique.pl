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


F <- comp(F,id2(a)).
F <- comp(id2(a),F).
comp(F,id(a)) = F :- type(F) = hom(A,a).
comp(id(a),F) = F :- type(F) = hom(a,B).

type(a) = ob.
type(id2(a)) = hom(a,a).
type(f) = hom(a,a).
?- id2(a) = id(a).
?- f = id2(a).
