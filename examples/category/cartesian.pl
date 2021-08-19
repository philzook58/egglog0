:- include(examples/category/symmetric.pl).

comp(dup(A), otimes(del(A), id(A)) <-> id(A).
comp(dup(A), otimes(id(A), del(A)) <-> id(A).
comp(dup(A), swap(A,A)) <-> dup(A).
comp(otimes(dup(A),dup(B)), otimes(id(A), swap(A,B), id(B))) <-> dup(otimes(A,B)).
comp(dup(A), otimes(id(A),dup(A)) <-> comp(dup(A), otimes(dup(A),id(A))).
del(otimes(A,B)) <-> otimes(del(A), del(B)).
dup(munit) = id(munit).
del(munit) = id(munit).

/* definition of proj and fan */
fan(F,G) <-> comp(dup(A), otimes(F,G)), type(F) = hom(A,B), type(G) = hom(A,C).

proj1(A,B) <-> otimes(id(A),del(B)).
proj2(A,B) <-> otimes(del(A),id(B)).

del(A) = X :-  comp(F, del(B)) = X, type(F) = hom(A,B).

/* or write in cod dom form */
comp(F,dup(A)) = X :- comp(dup(B), otimes(F,F)) = X, type(F) = hom(A,B).
comp(dup(B), otimes(F,F)) = X :- comp(F,dup(A)) = X, type(F) = hom(A,B).

/* this fle is incomplete */

/*

vec![rw!( "swap(munit(), munit()) => id(munit() oo munit())" ; "(swap munit munit)" => "(id (oo munit munit))" )],
rw!( "dup(a) . ((del)(a) om id(a)) == id(a)" ; "(. (dup ?a) (om (del ?a) (id ?a)))" <=> "(id ?a)" ),
rw!( "dup(a) . (id(a) om (del)(a)) == id(a)" ; "(. (dup ?a) (om (id ?a) (del ?a)))" <=> "(id ?a)" ),
rw!( "dup(a) . swap(a, a) == dup(a)" ; "(. (dup ?a) (swap ?a ?a))" <=> "(dup ?a)" ),
rw!( "(dup(a) om dup(b)) . ((id(a) om swap(a, b)) om id(b)) == dup(a oo b)" ; "(. (om (dup ?a) (dup ?b)) (om (om (id ?a) (swap ?a ?b)) (id ?b)))" <=> "(dup (oo ?a ?b))" ),
rw!( "dup(a) . (dup(a) om id(a)) == dup(a) . (id(a) om dup(a))" ; "(. (dup ?a) (om (dup ?a) (id ?a)))" <=> "(. (dup ?a) (om (id ?a) (dup ?a)))" ),
rw!( "(del)(a oo b) == (del)(a) om (del)(b)" ; "(del (oo ?a ?b))" <=> "(om (del ?a) (del ?b))" ),
rw!( "dup(munit()) == id(munit())" ; "(dup munit)" <=> "(id munit)" ),
rw!( "(del)(munit()) == id(munit())" ; "(del munit)" <=> "(id munit)" ),
vec![rw!( "pair(f, k) => dup(dom(type(f))) . (f om k)" ; "(pair ?f ?k)" => "(. (dup (dom (type ?f))) (om ?f ?k))" )],
rw!( "proj1(a, b) == id(a) om (del)(b)" ; "(proj1 ?a ?b)" <=> "(om (id ?a) (del ?b))" ),
rw!( "proj2(a, b) == (del)(a) om id(b)" ; "(proj2 ?a ?b)" <=> "(om (del ?a) (id ?b))" ),
vec![rw!( "f . (del)(b) => (del)(dom(type(f)))" ; "(. ?f (del ?b))" => "(del (dom (type ?f)))" )],
vec![rw!( "f . dup(b) => dup(dom(type(f))) . (f om f)" ; "(. ?f (dup ?b))" => "(. (dup (dom (type ?f))) (om ?f ?f))" )],
vec![rw!( "dup(a) . (f om f) => f . dup(cod(type(f)))" ; "(. (dup ?a) (om ?f ?f))" => "(. ?f (dup (cod (type ?f))))" )],
    
*/