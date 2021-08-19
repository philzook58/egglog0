/* https://proofwiki.org/wiki/Pullback_Lemma
  https://ncatlab.org/nlab/show/pasting+law+for+pullbacks
  
 */

type(id(A)) = hom(A,A) :- ob = type(A). 
/* ob = type(A) is probabnly slightly more efficient to search for than type(A) = ob */
F <- comp(id(A), F).
F <- comp(F, id(A)).

comp(F,id(B)) = F :- type(F) = hom(A,B).
comp(id(A),F) = F :- type(F) = hom(A,B).
/* associativity of composition */
comp(comp(F,G),H) <-> comp(F, comp(G,H)).
comp(F,comp(G,H)) <-> comp(F,G,H).
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


/*

  a <-H- d   p   a1 
F |      | K      Q
  v      v
  c <-G -b   J   e
*/

/* ideally users don't have to fill out this table.
It is obnoxious, obvious, and error prone.
*/
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
type(j) = hom(e,b).
type(p) = hom(a1,d).
type(q) = hom(a1,e). 


pullback(f,g,h,k).
pullback(k,j,p,q).


/*
Is big square a pullback?
1. Easy mode: does it commute:

I need to insert the convenient notation. 
I should just make it bidirectional.
Did increase the node count by quite a bit
comp(p,h,f).
comp(q,j,g).
*/
?- comp(p,h,f) = comp(q,j,g).

/*
2: Given another square

*/

/*
     -- r --            z

  a <-H- d   p   a1 
F |      | K      Q   w |
  v      v
  c <-G -b   J   e
*/
type(z) = ob.
type(r) = hom(z,a).
type(w) = hom(z,e).
comp(r,f) = comp(w,j,g). /* is square */
/* exists a morphism for which triangles commute */
?- comp(U,p,h) = r.
?- comp(U,q) = w.

/* and it is unique 
Is this right? or am I positing that the require morphism already exists with this?
I think the uniqueness of the eclass actually might do it.
That's interesting.
ALso there might be a 
(build morphism, write it down if you find it, now instanatiate it explcitly in this query) semanatics
?- u2 = univ(k,j, univ(f,g,r,comp(w,j)), w).
Eh that doesn't really matter does it?
Well it matters that it succeeded before I inserted this stuff maybe.

-? (comp p h f) = (comp q j g)
[];
-? (comp ?U p h) = r
[?U = (univ k j (univ f g r (comp w j)) w)];
-? (comp ?U q) = w
[?U = (univ k j (univ f g r (comp w j)) w)];




type(u2) = hom(z,a1).
comp(u2,comp(p,h)) = r.
comp(u2,q) = w.
?- u2 = U.



*/


