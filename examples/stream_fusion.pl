comp(map(F),map(G)) <-> map(comp(F,G)).
comp(filter(F), filter(G)) <-> filter(and(F,G)).
comp(rev,rev) <-> id.
comp(F,comp(G,H)) <-> comp(comp(F,G),H).
F <- comp(id,F).
F <- comp(F,id).

/* point free style or no? Is bare id ok? */

