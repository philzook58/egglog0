
:- include(examples/category/braided.pl).

type(a) = ob.
type(b) = ob.
type(c) = ob.
type(d) = ob.


rol(a, otimes(b,c,d)).

comp(  otimes(rol(a,b), id(c),    id(d)   ), 
       otimes(id(b),    rol(a,c), id(d)   ),
       otimes(id(b),    id(c),    rol(a,d))
 ).

?- rol(a, otimes(otimes(b,c),d)) =
comp(  otimes(rol(a,b), id(c),    id(d)   ), 
       otimes(id(b),    rol(a,c), id(d)   ),
       otimes(id(b),    id(c),    rol(a,d))
 ).

 /* id(otimes(a,b,c)). */
 comp( otimes(lor(a,b) , id(c)),
       otimes(id(b), lor(a,c) ),
       rol( otimes(b,c),  a )).

?-  id(otimes(a,b,c)) = 
 comp( otimes(lor(a,b) , id(c)),
       otimes(id(b), lor(a,c) ),
       rol( otimes(b,c),  a)).

?-  X = 
 comp( otimes(lor(a,b) , id(c)),
       otimes(id(b), lor(a,c) ),
       rol( otimes(b,c),  a)).

/*
maybe not.
type(cup(A)) = hom(munit, otimes(A,A)) :- ob = type(A).
type(cap(A)) = hom(otimes(A,A), munit) :- ob = type(A).
comp( otimes(id(A), cup(A)) , otimes(cap(A), id(A)) ) <-> id(A).
comp( otimes(cup(A), id(A)) , otimes(id(A), cap(A)) ) <-> id(A).
*/