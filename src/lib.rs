use egg::*;
use std::fmt::Write;

mod types;
pub use types::*;
use Entry::*;
use EqWrap::*;
use Term::*;

mod parser;
pub use parser::*;

fn merge_subst2(s1: &Subst, s2: &Subst) -> Option<Subst> {
    let mut s1 = s1.clone();
    for (v, i) in s2.vec.iter() {
        match s1.insert(*v, *i) {
            // Oh actually we should check
            Some(i1) => {
                if *i != i1 {
                    return None;
                }
            }
            None => (),
        }
    }
    return Some(s1);
}

fn merge_substs(s1s: &Vec<Subst>, s2s: &Vec<Subst>) -> Vec<Subst> {
    s1s.iter()
        .flat_map(|s1| s2s.iter().filter_map(move |s2| merge_subst2(s1, s2)))
        .collect()
}

struct MultiPattern<L> {
    patterns: Vec<EqWrap<Pattern<L>>>,
}

impl<L: Language, A: Analysis<L>> Searcher<L, A> for EqWrap<Pattern<L>> {
    fn search_eclass(&self, egraph: &EGraph<L, A>, eclass: Id) -> Option<SearchMatches> {
        match self {
            Bare(p) => p.search_eclass(egraph, eclass),
            Eq(p1, p2) => {
                let matches = p1.search_eclass(egraph, eclass)?;
                let matches2 = p2.search_eclass(egraph, eclass)?;
                let mut substs = vec![]; // this is merge substs above.
                for subst1 in &matches.substs {
                    for subst2 in &matches2.substs {
                        if let Some(subst) = merge_subst2(subst1, subst2) {
                            substs.push(subst);
                        }
                    }
                }
                if substs.len() == 0 {
                    None
                } else {
                    Some(SearchMatches { eclass, substs })
                }
            }
        }
    }
    fn vars(&self) -> Vec<egg::Var> {
        match self {
            Bare(p) => p.vars(),
            Eq(l, r) => {
                let mut vars = l.vars();
                vars.extend(r.vars());
                vars
            }
        }
    }
}

impl<L: Language, A: Analysis<L>> Searcher<L, A> for MultiPattern<L> {
    fn search_eclass(&self, egraph: &EGraph<L, A>, eclass: Id) -> Option<SearchMatches> {
        let mut iter = self.patterns.iter();
        let firstpat = iter.next()?;
        let searchmatches = firstpat.search_eclass(egraph, eclass)?;
        let mut matches = searchmatches.substs;
        for pat in iter {
            let mut temp_matches = vec![];
            for pmatch in pat.search(egraph) {
                temp_matches.append(&mut merge_substs(&matches, &pmatch.substs));
            }
            matches = temp_matches;
        }
        Some(SearchMatches {
            eclass,
            substs: matches,
        })
    }
    fn vars(&self) -> Vec<egg::Var> {
        let mut pats: Vec<_> = self
            .patterns
            .iter()
            .flat_map(|p| <Searcher<L,A>>::vars(p))
            .collect();
        pats.sort();
        pats.dedup();
        pats
    }
}

struct EqApply<L> {
    l: Pattern<L>,
    r: Pattern<L>,
}
// Hmm. Should I dfefine an applier for EqWrap<Pattern> instead?
impl<N, L> Applier<L, N> for EqApply<L>
where
    L: Language,
    N: Analysis<L>,
{
    fn apply_one(&self, egraph: &mut EGraph<L, N>, eclass: Id, subst: &Subst) -> Vec<Id> {
        // self.0.apply_one(egraph, eclass, subst)
        panic!("EqApply.apply_one was called");
    }

    // Could copy using apply_pat for better efficiency
    fn apply_matches(&self, egraph: &mut EGraph<L, N>, matches: &[SearchMatches]) -> Vec<Id> {
        let mut added = vec![]; // added are union updates, of which there are none
        for mat in matches {
            for subst in &mat.substs {
                // This should be ok because we know they are patterns. Not very safe.
                let id1 = self.l.apply_one(egraph, 0.into(), subst)[0];
                let id2 = self.r.apply_one(egraph, 0.into(), subst)[0];
                let (to, did_something) = egraph.union(id1, id2);
                if did_something {
                    added.push(to)
                }
            }
        }
        added
    }

    fn vars(&self) -> Vec<egg::Var> {
        let mut vars = self.l.vars();
        vars.extend(self.r.vars());
        vars
    }
}

// Could probably generalize from pattern.
struct IgnoreApply<L>(Pattern<L>);

impl<N, L> Applier<L, N> for IgnoreApply<L>
where
    L: Language,
    N: Analysis<L>,
{
    fn apply_one(&self, egraph: &mut EGraph<L, N>, eclass: Id, subst: &Subst) -> Vec<Id> {
        self.0.apply_one(egraph, eclass, subst)
    }

    // TODO: Could copy using apply_pat from Pattern impl for better efficiency. Need to make it public?
    fn apply_matches(&self, egraph: &mut EGraph<L, N>, matches: &[SearchMatches]) -> Vec<Id> {
        // let mut added = vec![]; // added are union updates, of which there are none
        for mat in matches {
            for subst in &mat.substs {
                self.apply_one(egraph, 0.into(), subst); // root is just ignored?
            }
        }
        // TODO: REALLY THINK ABOUT THIS!!!!
        vec![0.into()] // so a clause may not make more stuff happen. Early saturation.
    }

    fn vars(&self) -> Vec<egg::Var> {
        self.0.vars()
    }
}

/*
MultiApplier could be useful / efficient
Using Conditional Equals could be useful if all variables known
But really getting patterns to compile with substituion pieces considered known subsumes this optimization
I think in modern egg with yihong's optimization.
Sanity checks that needed variables exist would be good.
May want to run Runner multiple times since it may not get restarted. Currently I have that vec![0] hack
_ for dummy variables
The ability to check to see if something is in the egraph.
graphviz dumping the egraph
harrop formula
merge_subst that doesn't copy?
Give rules names. Keep a hash table of them?
Queries with variables
Queries should be conjunctions 
a REPL would be sweet. especially if we have higher order rules, we could watch the database, add queries
termination based on the query condition
side effectful searchers and appliers (printing mostly), functions.
Astsize with weighting? Does that get me anywhere?
infix operators
*/

fn run_file(file: Vec<Entry>) -> String {
    let mut queries = vec![];
    let mut rules = vec![];
    let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    for entry in file {
        match entry {
            Clause(head, body) => {
                let body = body
                    .iter()
                    .map(|eqt| match eqt {
                        Bare(p) => Bare(pattern_of_term(p)),
                        Eq(a, b) => Eq(pattern_of_term(a), pattern_of_term(b)), 
                    })
                    .collect();
                let searcher = MultiPattern { patterns: body };
                let applier = match head {
                    Bare(head) => {
                        let applier = IgnoreApply(pattern_of_term(&head));
                        rules.push(egg::Rewrite::new("", searcher, applier).unwrap());
                    }
                    Eq(l, r) => {
                        let l = pattern_of_term(&l);
                        let r = pattern_of_term(&r);
                        let applier = EqApply { l, r };
                        rules.push(egg::Rewrite::new("", searcher, applier).unwrap());
                    }
                };
            }
            Query(mut qs) => queries.append(&mut qs),
            Directive(_d) => (),
            BiRewrite(a, b) => {
                let a = pattern_of_term(&a);
                let b = pattern_of_term(&b);
                rules.push(egg::Rewrite::new("", a.clone(), b.clone()).unwrap());
                rules.push(egg::Rewrite::new("", b, a).unwrap());
            }
            Rewrite(a, b, body) => {
                let applier = pattern_of_term(&a);
                let b = pattern_of_term(&b);
                // consider shortcircuiting case where body = []
                let conditions: Vec<_> = body
                    .iter()
                    .map(|e| {
                        let (l, r) = match e {
                            Eq(a, b) => (pattern_of_term(&a), pattern_of_term(&b)),
                            Bare(a) => (pattern_of_term(&a), pattern_of_term(&a)),
                        };
                        ConditionEqual(l, r)
                    })
                    .collect();
                let condition = move |egraph: &mut EGraph<_, _>, eclass: Id, subst: &Subst| {
                    conditions.iter().all(|c| c.check(egraph, eclass, subst))
                };
                let applier = ConditionalApplier { condition, applier };
                rules.push(egg::Rewrite::new("", b, applier).unwrap());
            }
            Fact(Eq(a, b)) => {
                let a_id = eid_of_groundterm(&mut egraph, &a);
                let b_id = eid_of_groundterm(&mut egraph, &b);
                egraph.union(a_id, b_id);
            }
            Fact(Bare(a)) => {
                eid_of_groundterm(&mut egraph, &a);
            }
        }
    }

    let mut runner = Runner::default().with_egraph(egraph).run(&rules);
    runner.print_report();
    // runner.egraph.dot().to_png("target/foo.png").unwrap();
    let mut buf = String::new();
    for q in queries {
        match q {
            Bare(a) => {
                if let Some(a) = is_ground(&a) {
                    let root = eid_of_groundterm(&mut runner.egraph, &a);
                    let mut extractor = Extractor::new(&runner.egraph, AstSize);
                    let (_best_cost, best) = extractor.find_best(root);
                    writeln!(buf, "{} => {}", a, best);
                }
            }
            Eq(a, b) => {
                // Could run pattern search on the egraph and extract minimal terms if non ground query.
                if let Some(a) = is_ground(&a) {
                    let aid = eid_of_groundterm(&mut runner.egraph, &a);
                    if let Some(b) = is_ground(&b) {
                        let bid = eid_of_groundterm(&mut runner.egraph, &b);
                        if aid == bid {
                            writeln!(buf, "{} = {}", a, b);
                        } else {
                            writeln!(buf, "{} ?= {}", a, b);
                        }
                    }
                }
            }
        }
    }
    buf
}

pub fn run(s: String) -> Result<String, String> {
    let f = parse_file(s)?;
    Ok(run_file(f))
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run_wasm(s: String) -> String {
    match run(s) {
        Ok(e) => e,
        Err(e) => e,
    }
}
