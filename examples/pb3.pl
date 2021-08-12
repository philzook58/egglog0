type(id(A)) = hom(A,A) :- ob = type(A). 
/* ob = type(A) is probabnly slightly more efficient to search for than type(A) = ob */
F <- comp(id(A), F).
F <- comp(F, id(A)).

comp(F,id(B)) = F :- type(F) = hom(A,B).
comp(id(A),F) = F :- type(F) = hom(A,B).
/* associativity of composition */
comp(comp(F,G),H) <-> comp(F, comp(G,H)).
comp(F,comp(G,H)) <- comp(F,G,H).
/* Composition exists if types work out */
type(comp(F,G)) = hom(A,C) :- hom(A,B) = type(F), hom(B,C) = type(G).

/* pullback is square */
comp(H,F) = comp(K,G) :- pullback(F,G,H,K).


/* universal morphism exists. triangles 
is univ a function of H1 K1?
*/
comp(univ(F,G,H,K),H1) = H,
comp(univ(F,G,H,K),K1) = K,
type(univ(F,G,H,K)) = hom(Z,E)
:- pullback(F,G,H1,K1), comp(H,F) = comp(K,G), type(F) = hom(A,B), type(H) = hom(Z,A), type(H1) = hom(E,A).

/* unique */
univ(F,G,H,K) = U :- pullback(F,G,H1,K1), comp(H,F) = comp(K,G), H = comp(U,H1), K = comp(U,K1).

G = K :- monic(F), comp(G,F) = comp(K,F).
/*

  a <-H- d   p   a1 
F |      | K      Q
  v      v
  c <-G -b   J   e
*/

/* ideally users don't have to fill out this table */
type(a) = ob.
type(b) = ob.
type(c) = ob.
type(d) = ob.
type(e) = ob.
type(a1) = ob.


type(f) = hom(a,c).
type(g) = hom(b,c).
type(h) = hom(d,a).
type(k) = hom(d,b).
/*
type(j) = hom(e,b).
type(p) = hom(a1,d).
type(q) = hom(a1,e). 
*/
monic(f).


pullback(f,g,h,k).

/*
Well defined-ness of comp. Convenient for lowering the type annotation requirements. An actual typechker would be desirable
in my opinion since types are easy.

type(dom(f)) = ob,
type(cod(f)) = ob,
type(f) = hom(dom(f),cod(f)),
cod(F) = dom(G),
type(dom(G)) = ob,
type(cod(G)) = ob,
type(G) = hom(dom(G),cod(G)),
 :- comp(F,G).

monic(F) :- forall(g,h, comp(g,F) = comp(h,F) => g = h ).
?- monic(h).
*/
/* pullback(k,j,p,q). 


comp(p,h,f).
comp(q,j,g).
?- comp(p,h,f) = comp(q,j,g).
*/


type(z) = ob.
type(p) = hom(z,d).
type(q) = hom(z,d).
comp(p,k) = comp(q,k).

?- p = q.
?- f = g.
?- p = f.
?- k = h.
?- k = g.
?- type(comp(p,h)) = T.
?- type(id(a)) = hom(a,a).
?- comp(comp(id(a), h), k) = T.
