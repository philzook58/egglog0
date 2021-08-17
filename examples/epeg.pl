/* https://arxiv.org/pdf/1012.1802.pdf
appendix axiosms.


*/
A = T :- theta(L, A, T) = T.
apply(F, ite(C,A,B)) <-> ite(C,apply(F,A),apply(F,B)).
/* I call phi noes ite */
A <- ite(C,A,A).
ite(C,A,E) <- ite(C,ite(C,A,B),E).
/* These are not axioms in the index? */
A <- ite(true,A,B).
B <- ite(false, A, B).


apply(F, ite(C,A,B)) <-> ite(C,apply(F,A),apply(F,B)).

phi()