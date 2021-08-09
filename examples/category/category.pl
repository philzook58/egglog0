/* Identities exist */
type(id(A)) = hom(A,A) :- ob = type(A). 
/* Composition exists if types work out */
type(comp(F,G)) = hom(A,C) :- hom(A,B) = type(F), hom(B,C) = type(G).
/* Identity Absorption */
F <- comp(id(A), F).
F <- comp(F, id(A)).
/* associativity of composition */
comp(comp(F,G),H) <-> comp(F, comp(G,H)).

/* convenience. These would be better as macros? */
comp(F,comp(G,H)) <- comp(F,G,H).
comp(F,comp(G,comp(H,K))) <- comp(F,G,H,K).