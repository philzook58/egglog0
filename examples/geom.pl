/* https://en.wikipedia.org/wiki/List_of_first-order_theories */

/* exists!(C, on(A,C), on(B,C)):- distinct(A,B) */
on(A, line(A,B)), on(B, line(A,B)) :- point(A), distinct(A,B), point(B). 
F = line(A,B) :-  point(A), point(B), distinct(A,B), on(A,L), on(B,L). /* unique */

/* veblen axiom */

/* 3 distinct points */
distinct(p1(L), p2(L)),
distinct(p2(L), p3(L)), 
distinct(p1(L), p3(L)),
on(p1(L), L),
on(p2(L), L),
on(p3(L), L), :-
line(L).

distinct(B,A) :- distinct(A,B).
 /* equal?  use <- ? */

/* better not have a contradiction */
contradiction :- distinct(A,A).

point(a).
line(l).


/* hmm I could have just run cpp rather than done this stuff myself. 
Of course, that'd make web compiling not works so good */


