:- include(examples/category/monoidal.pl).

type(a) = ob.
type(b) = ob.
type(c) = ob.
/* simplify to id */
?- comp(id(a),id(a)) = F.
comp(id(a),otimes(id(a), id(munit))).
?- comp(id(a),otimes(id(a), id(munit))) = F.

type(f) = hom(a,b).
otimes(f,f,f).
?- otimes(f,f,f) = otimes(otimes(f,f),f).
otimes(f,f,id(munit),f).

?- otimes(f,f,id(munit),f) = otimes(otimes(f,f),f).

