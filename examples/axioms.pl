

/* Base equality facts */
Axiom ax1 : y = x.
Axiom ax2 : z = y.
/* Base egraph insertions. No implied equalities. For "seeding" the egraph. */
Axiom ax3 : f x.
Axiom ax4 : bar boo.
Axiom ax7 : plus p r.
Axiom ax9 : forall x e, f (f (f x)) = e => e = x.
Axiom ax10 : forall x e, f (f x) = e => e = x.
/* general looking prolog-like rules */
Axiom fizbo : forall x, f x = q => bar x.
Axiom fozbo : forall x z, fizzy floozy /\ buppo z  => biz z = baz (biz boz).

|- f x = x.
|- x = x.
|- y = x.
|- plus p r = plus r p.
|- junk boo  = otherjunk baz.
|- exists z, f z = x.
?- f(x) = x.
?- x = x.
?- y = x.
?- plus(p,r) = plus(r,p).
?- junk(boo) = otherjunk(baz).