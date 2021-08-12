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

/* all pullbacks exist */
type(pb1(F,G)) = hom( pbo(F,G), A ),
type(pb2(F,G)) = hom( pbo(F,G), B ),
type(pbo(F,G)) = ob,
comp(pb1(F,G),F) = comp(pb2(F,G),G)
:- hom(A,C) = type(F), hom(B,C) = type(G).



/* triangles */
comp(univ(F,G,H,K),pb1(F,G)) = H,
comp(univ(F,G,H,K),pb2(F,G)) = K,
type(univ(F,G,H,K)) = hom(Z,pbo(F,G))
:- comp(H,F) = comp(K,G), type(F) = hom(A,B), type(H) = hom(Z,A).

/* unique */
univ(F,G,H,K) = U :- comp(H,F) = comp(K,G), H = comp(U,pb1(F,G)), K = comp(U,pb2(F,G)).
/*

  a <-H-    
F |      | K
  v      v
  c <-G -b   J   d
*/
/* pb1(id(A),id(A)) */
type(a) = ob.
/* type(b) = ob.
type(c) = ob. */
/* type(d) = ob. */

/* type(f) = hom(a,c).
type(g) = hom(b,c). */
/* type(j) = hom(d,b). */
?- comp(id(a),id(a)) = id(a).
?- f = id(a).
?- comp(pb1(f,g),f) = comp(pb2(f,g),g).
?- pb1(id(a), id(a)) = id(a). /* Hmm. actually this is not a theorem */


/* Why? A useful canoncial choice to prevent explosion. But does*/
pbo(id(a),id(a)) = a.
pb1(id(a),id(a)) = id(a).
?- pb2(id(a),id(a)) = id(a). 
?- pbo(id(a),id(a)) = a.