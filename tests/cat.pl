
/* A <- dom(hom(A,B)).
B <- cod(hom(A,B)).
hom(A,A) <- type(id(A)).
hom(dom(type(F)), cod(type(G))) <- type(comp(F,G)).
*/

type(id(A)) = hom(A,A) :- ob = type(A). 
/* Composition exists if types work out */
type(comp(F,G)) = hom(A,C) :- hom(A,B) = type(F), hom(B,C) = type(G).
F <- comp(id(A), F).
F <- comp(F, id(A)).
/* associativity of composition */
comp(comp(F,G),H) <-> comp(F, comp(G,H)).

/* Monoidal */
type(otimes(A,B)) = ob :- ob = type(A), ob = type(B).
type(otimes(F,G)) = hom(otimes(A,B),otimes(C,D)) :- hom(A,C) = type(F), hom(B,D) = type(G).

/* The non-generative form */
hom(otimes(A,B),otimes(C,D)) = T :- type(otimes(F,G))  = T, hom(A,C) = type(F), hom(B,D) = type(G).


/* Covers the object case too */
otimes(otimes(F,G),H) <-> otimes(F, otimes(G,H)).

type(munit) = ob.
A <- otimes(munit, A).
A <- otimes(A,munit).
F <- otimes(id(munit), F).
F <- otimes(F, id(munit)).

/* Is this one necessary? */
id(otimes(A,B)) <-> otimes(id(A), id(B)).

comp(otimes(F,G), otimes(P,Q)).


/* Now this is possible 
F = comp(id(A), F) :- hom(A,B) = type(F).
F = comp(F, id(B)) :- hom(A,B) = type(F).
*/
/* 
More efficient form 
*/

type(a) = ob.
/* Uhhh. Hmm. */
type(id(A)) = hom(A,A) :- type(A) = ob.

type(id(a)).
type(comp(id(a),id(a))).
type(f) = hom(a,b).
type(comp(id(a), f)).
/*
Should we be using the tuff in the query injected into the database?

 */
?- type(id(a)), type(comp(id(a),f)).
/*
    let mut rules : Vec<Rewrite<CatLanguage, ()>> = vec![
        vec![rw!( "dom(hom(a, b)) => a" ; "(dom (hom ?a ?b))" => "?a" )],
        vec![rw!( "cod(hom(a, b)) => b" ; "(cod (hom ?a ?b))" => "?b" )],
        vec![rw!( "type(id(a)) => hom(a, a)" ; "(type (id ?a))" => "(hom ?a ?a)" )],
        vec![rw!( "type(f . g) => hom(dom(type(f)), cod(type(g)))" ; "(type (. ?f ?g))" => "(hom (dom (type ?f)) (cod (type ?g)))" )],
        vec![rw!( "type(f om g) => hom(dom(type(f)) oo dom(type(g)), cod(type(f)) oo cod(type(g)))" ; "(type (om ?f ?g))" => "(hom (oo (dom (type ?f)) (dom (type ?g))) (oo (cod (type ?f)) (cod (type ?g))))" )],
        vec![rw!( "type(a oo b) => :ob" ; "(type (oo ?a ?b))" => "ob" )],
        vec![rw!( "type(munit()) => :ob" ; "(type munit)" => "ob" )],
        vec![rw!( "type(swap(a, b)) => hom(a oo b, b oo a)" ; "(type (swap ?a ?b))" => "(hom (oo ?a ?b) (oo ?b ?a))" )],
        vec![rw!( "type((del)(a)) => hom(a, munit())" ; "(type (del ?a))" => "(hom ?a munit)" )],
        vec![rw!( "type(dup(a)) => hom(a, a oo a)" ; "(type (dup ?a))" => "(hom ?a (oo ?a ?a))" )],
        vec![rw!( "type(pair(f, g)) => hom(dom(type(f)), cod(type(f)) oo cod(type(g)))" ; "(type (pair ?f ?g))" => "(hom (dom (type ?f)) (oo (cod (type ?f)) (cod (type ?g))))" )],
        vec![rw!( "type(proj1(a, b)) => hom(a oo b, a)" ; "(type (proj1 ?a ?b))" => "(hom (oo ?a ?b) ?a)" )],
        vec![rw!( "type(proj2(a, b)) => hom(a oo b, b)" ; "(type (proj2 ?a ?b))" => "(hom (oo ?a ?b) ?b)" )],
        vec![rw!( "f . id(b) => f" ; "(. ?f (id ?b))" => "?f" )],
        vec![rw!( "id(a) . f => f" ; "(. (id ?a) ?f)" => "?f" )],
        vec![rw!( "a oo munit() => a" ; "(oo ?a munit)" => "?a" )],
        vec![rw!( "munit() oo a => a" ; "(oo munit ?a)" => "?a" )],
        rw!( "f . (g . h) == (f . g) . h" ; "(. ?f (. ?g ?h))" <=> "(. (. ?f ?g) ?h)" ),
        vec![rw!( "id(munit()) om f => f" ; "(om (id munit) ?f)" => "?f" )],
        vec![rw!( "f om id(munit()) => f" ; "(om ?f (id munit))" => "?f" )],
        rw!( "a oo (b oo c) == (a oo b) oo c" ; "(oo ?a (oo ?b ?c))" <=> "(oo (oo ?a ?b) ?c)" ),
        rw!( "f om (h om j) == (f om h) om j" ; "(om ?f (om ?h ?j))" <=> "(om (om ?f ?h) ?j)" ),
        rw!( "id(a oo b) == id(a) om id(b)" ; "(id (oo ?a ?b))" <=> "(om (id ?a) (id ?b))" ), 
        vec![rw!( "(f . g) om (p . q) => (f om p) . (g om q)" ; "(om (. ?f ?g) (. ?p ?q))" => "(. (om ?f ?p) (om ?g ?q))" )],
        rw!( "swap(a, b) . swap(b, a) == id(a oo b)" ; "(. (swap ?a ?b) (swap ?b ?a))" <=> "(id (oo ?a ?b))" ),
        rw!( "(swap(a, b) om id(c)) . (id(b) om swap(a, c)) == swap(a, b oo c)" ; "(. (om (swap ?a ?b) (id ?c)) (om (id ?b) (swap ?a ?c)))" <=> "(swap ?a (oo ?b ?c))" ),
        rw!( "(id(a) om swap(b, c)) . (swap(a, c) om id(b)) == swap(a oo b, c)" ; "(. (om (id ?a) (swap ?b ?c)) (om (swap ?a ?c) (id ?b)))" <=> "(swap (oo ?a ?b) ?c)" ),
        rw!( "swap(a, munit()) == id(a)" ; "(swap ?a munit)" <=> "(id ?a)" ),
        rw!( "swap(munit(), a) == id(a)" ; "(swap munit ?a)" <=> "(id ?a)" ),
       
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
                
        ].concat();
        */