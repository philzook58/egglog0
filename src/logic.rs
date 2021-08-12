
use crate::*;
// G and D formula? My intend is for this to be query formula for the moment.
#[derive(Debug, PartialEq)]
pub enum Formula {
    Implies(Box<Formula>, Box<Formula>),
    Conj(Vec<Formula>),
    ForAll(String, Box<Formula>),
    Exists(String, Box<Formula>),
    Atom(EqWrap<Term>),
}

enum GoalFormula {
    ForAll(String, Box<GoalFormula>), // Introduce fresh variable.
    Exists(String, Box<Formula>), // Pattern variable. does not need skolemization. pattern variable can only be instanniteated with already instroduced variables
    // ExistsUnique ? == exists b, (P(b) /\ forall c, P(c) => c = b), and b can't contain c. oof. That's a toughy. Doable as an analysis.
    Conj(Vec<GoalFormula>), // we can normalize to disj of conj form? Also we could just build out the matching language.
    Disj(Vec<GoalFormula>),
    Implies(ProgramFormula, Box<GoalFormula>),
    Atom(EqWrap<Term>)
}

enum PatternFormula {
    Conj(Vec<PatternFormula>),
    Disj(Vec<PatternFormula>),
    Exists(String, Box<PatternFormula>),
    Atom(EqWrap<Term>)
}

// Perhaps if I'm willing to accept higher order appliers that insert into the rule set
// I can combine some of these.
enum ApplierFormula {
    Conj(Vec<ApplierFormula>),
    Exists(String, Box<ApplierFormula>), // gensym semantics. in context of all foralls?
    Atom(EqWrap<Term>)
}

type AtomicFormula = EqWrap<Term>;

// is programformula the same as applierformula and goalformula the same as patternformual

enum ProgramFormula {
 /*   Implies(PatternFormula, ProgramFormula ),
    //Forall(String, Box<ProgramFormula> ), // No not fine. foo(X,Y,Z) needs to be disallowed.
    Apply(ApplierFormula)
    Conj(Box<ProgramFormula>, Box<ProgramFormula>),
    // Exists // gensym semantics? exists a b c, foo(a,b,c). seems fine.
    // ForallBounded? it's like forall x : [0..10], or something. we need to know where to pull from
    ForAllScoped( Vec<String>, PatternFormula, ApplierFormula) // Requires a pattern formula that contains all the variables.
*/
}
/*
ConjPattern = MultiPattern<>
DisjPattern = OrPattern<>
ematch {
    matches = vec![]
    for pat in patterns {
        matches.extend(pat.match)
    }
}
vars = intersction of vars of patterns rather than union


datalog over a hash cons would be a weaker egglog. Interesting to think about.


compiled form



Impl(ReWrite)

*/
