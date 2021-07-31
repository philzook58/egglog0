a(x).
q(x).
b(X) :- a(X), q(X).
c(X) <- b(X).
?- c(x) = b(x), a(x) = b(x).
