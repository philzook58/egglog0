:- include(examples/category/monoidal.pl).
/* right over left = rol */
/* left over right = lor */
/*
type(rol(A,B)) = hom(otimes(A,B), otimes(B,A)) :- ob = type(A), ob = type(B).
type(lor(A,B)) = hom(otimes(A,B), otimes(B,A)) :- ob = type(A), ob = type(B).
*/
id(otimes(A,B)) <-> comp(rol(A,B), lor(B,A)).
id(otimes(A,B)) <-> comp(lor(A,B), rol(B,A)).

id(A) <-> rol(A, munit).
id(A) <-> rol(munit,A).

id(A) <-> lor(A, munit).
id(A) <-> lor(munit,A).


/* naturality */
/* These need guarding
Consider allowing bidirectional guards.
Hmm. These are discovery not ConditionalEqual.
Is this the moment I need to consider making multipattern matching work better?
 */
comp(otimes(F,G), rol(B,D)) = X :- comp(rol(C,A), otimes(G,F)) = X, hom(A,B) = type(F), hom(C,D) = type(G).
comp(rol(C,A), otimes(G,F)) = X :- comp(otimes(F,G), rol(B,D)) = X, hom(A,B) = type(F), hom(C,D) = type(G).


rol(A, otimes(B,C)) <-> comp(  otimes(rol(A,B), id(C)), otimes(id(B), rol(A,C))).
rol(otimes(A,B), C) <-> comp( otimes(id(A), rol(B,C)), otimes(rol(A,C), id(B))).
/* and likewise for lor
Given they are inverses do the other equations follow?
 */

comp(otimes(F,G), lor(B,D)) = X :- comp(lor(C,A), otimes(G,F)) = X, hom(A,B) = type(F), hom(C,D) = type(G).
comp(lor(C,A), otimes(G,F)) = X :- comp(otimes(F,G), lor(B,D)) = X, hom(A,B) = type(F), hom(C,D) = type(G).


lor(A, otimes(B,C)) <-> comp( otimes(lor(A,B), id(C)), otimes(id(B), lor(A,C))).
lor(otimes(A,B), C) <-> comp( otimes(id(A), lor(B,C)), otimes(lor(A,C), id(B))).