:- include(examples/category/monoidal.pl).

id(A) <- swap(A, munit). 
id(A) <- swap(munit,A).
id(otimes(A,B)) <- comp(swap(A,B), swap(B,A)).