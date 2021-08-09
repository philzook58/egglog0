/* Straight from https://en.wikipedia.org/wiki/Datalog */
parent(xerces, brooke).
parent(brooke, damocles).
ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :- parent(X, Z), ancestor(Z, Y).
?- ancestor(xerces, X).


/* https://www.stephendiehl.com/posts/exotic04.html */
/* All men are mortal / valar morghulis */
mortal(X) :- human(X).

/* Socrates is a man. */
human(socrates).

/* Is Socrates mortal? */
?- mortal(socrates).

/* https://souffle-lang.github.io/simple */

path(X, Y) :- edge(X, Y).
path(X, Y) :- path(X, Z), edge(Z, Y).
edge(a,b).
edge(b,c).

?- path(X,Y).