/*
**************************************************************************
**************************************************************************
  _                       _                             ______            
 | |                     (_)                           |  ____|           
 | |     ___   __ _  __ _ _ _ __   __ _    __ _ _ __   | |__   __ _  __ _ 
 | |    / _ \ / _` |/ _` | | '_ \ / _` |  / _` | '_ \  |  __| / _` |/ _` |
 | |___| (_) | (_| | (_| | | | | | (_| | | (_| | | | | | |___| (_| | (_| |
 |______\___/ \__, |\__, |_|_| |_|\__, |  \__,_|_| |_| |______\__, |\__, |
               __/ | __/ |         __/ |                       __/ | __/ |
              |___/ |___/         |___/                       |___/ |___/ 
*************************************************************************
**************************************************************************
                           Datalog on E-Graphs

> Philip Zucker
> pzucker@draper.com
> Draper Laboratory



-------
Datalog
-------

> Bottom up relative of prolog
> Databases, Logic, and Proofs

*/

/* Facts */
edge(a,b).
edge(b,c).
edge(c,d).

/* Rules */
path(X,Y) :- edge(X,Y).
path(X,Z) :- edge(X,Y), path(Y,Z).

/* Query */
?- path(X,Y).

/*
-------
Egglog0
-------
> Inspired by Relational E-Matching
> E-Graphs are a Database
> This database holds terms and equality relation
> Supports ordinary datalog with terms
> Rules: query using RHS (e-matching multipattern), instantiate and insert LHS
> Capitalized pattern variables bind to eclass ids
> Special equality `_=_` is E-graph equality / union find
> Queries: e-match and return all results.
*/

X = E :- add(X,zero) = E.
add(Y,X) = E :- add(X,Y) = E.
add(Y,X) <- add(X,Y).  /* identical syntax sugar */

add(zero,a).
?- add(zero,a) = Z.

/*
-------------
Multipatterns
-------------
> Multipatterns bind, Guards check.
> Threads the e-matching compiler environment binding between patterns
> Upstreamed to egg https://github.com/egraphs-good/egg/pull/168

*/












/*
***********************************************
***********************************************
  ______                           _           
 |  ____|                         | |          
 | |__  __  ____ _ _ __ ___  _ __ | | ___  ___ 
 |  __| \ \/ / _` | '_ ` _ \| '_ \| |/ _ \/ __|
 | |____ >  < (_| | | | | | | |_) | |  __/\__ \
 |______/_/\_\__,_|_| |_| |_| .__/|_|\___||___/
                            | |                
                            |_|        
***********************************************
***********************************************
*/










/*
-----------
Injectivity
-----------

> ∀ a b, f(a) = f(b) -> a = b
> ∀ a b, a != b -> f(a) != f(b)
> Constructors, Negation, constant addition
> Unification

*/
X = Y, Xs = Ys :- cons(X,Xs) = cons(Y,Ys).
X = Y :- add(X,Z) = add(Y,Z).








/*
---------------------
Memory Simplification
---------------------

> Alias Analysis + Simplification
> SMTlib theory of arrays %/ McCarthy
> Many SMT theories are expressible as Horn Clauses (side conditions)
*/

/*select grabs stored value*/
V <- select(A ,store(A, V, Mem)).

/*select ignores different addresses*/
select(A1,Mem) = E :- select(A1,store(A2,V,Mem)) = E, neq(A1,A2).

/*non aliasing writes commute*/
store(A2,V2, store(A1,V1,Mem)) = E :- store(A1,V1, store(A2,V2,Mem)) = E, neq(A1,A2).

/*Aliasing Writes destroy old value.*/
store(A, V1, Mem) <- store(A, V1, store(A,V2,Mem)).

/*
----------------
Equation Solving
----------------
> Variable Isolation
> Extract terms without variables
*/

sub(Z,X) = Y :- add(X,Y) = Z.












/*
----------
Reflection
----------
> Hypothetical reasoning
> Boolean algebraic reasoning
*/
A = B :- true = eq(A,B).
true = eq(A,B) :- A = B.   











/*
-------------------------
Uniqueness Quantification
-------------------------
> Common in universal constructions in category theory
> Skolemize existentials 
                `∀ x, P(x) -> ∃ y, Q(x,y)`  becomes
                `∀ x, P(x) -> Q(x,f(x))`
> Uniqueness Property `∀ a b, P(a) /\ P(b) -> a = b`
  is directly expressible in Egglog0.
> See "Pullback of Monic is Monic" and 
  "Composition of Pullbacks" examples for more detail












***************************************************
***************************************************
   ____                  _   _                ___  
  / __ \                | | (_)              |__ \ 
 | |  | |_   _  ___  ___| |_ _  ___  _ __  ___  ) |
 | |  | | | | |/ _ \/ __| __| |/ _ \| '_ \/ __|/ / 
 | |__| | |_| |  __/\__ \ |_| | (_) | | | \__ \_|  
  \___\_\\__,_|\___||___/\__|_|\___/|_| |_|___(_)  
***************************************************
*************************************************** 
> Thanks to Yihong Zhang, Remy Yisu Wang, Max Willsey, 
        Zachary Tatlock, Alessandro Cheli, Cody Roux,
        James Fairbanks, and Evan Patterson for their 
        helpful discussions.
> DARPA Grant #

------------
Related Work
------------
> Relational E-Matching https://arxiv.org/abs/2108.02290
> SMT Multipatterns
> Souffle Egg https://www.hytradboi.com/2022/writing-part-of-a-compiler-in-datalog
> Egg-lite
*/