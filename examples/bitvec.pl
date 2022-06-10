


add(len(A), len(B)) <- len(concat(A,B)).

/* if you try to extract more than is there, blow up world. Or maybe assume wraparound? */
Size <- len(extract(Start, Size, A)).

A <- extract(zero, len(A), concat(A,B)).
B <- extract(len(A), len(B), concat(A,B)).


/*
bitvector arith
*/
B <- bvadd(bvzero(N),B).




/*
axioms of arithemtic
*/