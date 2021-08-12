
/* Does it make sense to suppress the types here. Do I have a model? Partial functions? Relations? */
R <-> comp(id, R).
R <-> comp(R,id).

G <- comp(snd,fan(F,G)).
F <- comp(fst,fan(F,G)).
/* bifunctor laws? Are these necessary? */
comp(fan(F,G),H) <-> fan(comp(F,H),comp(G,H)).

comp(F,comp(G,H)) <-> comp(comp(F,G),H).
comp(F,G,H) <-> comp(F,comp(G,H)).

dup = fan(id,id).
swap = fan(snd,fst).
par(F,G) <-> fan(comp(F,fst),comp(G,snd)).

par(f,par(g,h)).
par(par(f,g),h).
comp(snd,par(f,par(g,h))) .
comp(par(h,h),fan(f,g)).
fan(comp(h,f),comp(h,g)).
comp(par(f,g),par(h,k)).
par(comp(f,h), comp(g,k)).

?- comp(par(h,h),fan(f,g)) = fan(comp(h,f),comp(h,g)).
?- comp(par(f,g),par(h,k)) = par(comp(f,h), comp(g,k)).
?- comp(snd,par(f,par(g,h))) = A.
?- f = g. /* sanity check */


/* comp(split(F,G), left) */

/* curry uncurry apply */
