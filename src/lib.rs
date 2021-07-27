use egg::*;
use std::fmt::Write;

mod types;
pub use types::*;
use Entry::*;
use EqWrap::*;
use Term::*;

mod parser;
pub use parser::*;
/*

enum HarropGoal {
  ForAll()
  Exists()
}
enum Harrop {

}
enum QueryResult {

}

*/
/*
fn merge_subst(s1: &mut Subst, s2: &Subst) -> bool {
    for (v, i) in s2.vec.iter() {
        if let None = s1.insert(*v, *i) {
            return false;
        }
    }
    return true;
}
*/
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

// Note: The new ground checks ought to be useful for multipattern search
struct MultiPattern<L> {
    patterns: Vec<Pattern<L>>, // EqWrap it eventually. To complicated for now  //dyn Searcher<L,A> // smallvec?
}

use std::iter;
// Or should this be part of Applier? Feels odd

impl<L: Language, A: Analysis<L>> Searcher<L, A> for MultiPattern<L> {
    fn search_eclass(&self, egraph: &EGraph<L, A>, eclass: Id) -> Option<SearchMatches> {
        let mut iter = self.patterns.iter();
        let firstpat = iter.next()?;
        let searchmatches = firstpat.search_eclass(egraph, eclass)?;
        let mut matches = searchmatches.substs;
        for pat in iter {
            let mut temp_matches = vec![];
            for pmatch in pat.search(egraph){
                temp_matches.append(&mut merge_substs(& matches, &pmatch.substs));
            }
            matches = temp_matches;
        }
        Some(SearchMatches {
            eclass,
            substs: matches,
        })
    }

    // for p in &self.patterns {

    /* match p {
    Bare(p) => { let pmatch = p.search_eclass(egraph, eclass);
                            for m in matches{
                                for pm in pmatch {

                                    merge_subst(&mut m.subst, pm.subst);
                                }
                    }},
    Eq(p1,p2) => ()
                } */
    // }

    /*
        fn search(&self, egraph: &EGraph<L, A>) -> Vec<SearchMatches> {
            let mut matches = vec![];
            for p in self.patterns {
                match p {
                    Bare(p) => {    let pmatch = p.search(egraph);
                                            for m in matches{
                                                for pm in pmatch {

                                                    merge_subst(&mut m.subst, pm.subst);
                                                }
                                    }},
                    Eq(p1,p2) => () /*
                                    let pmatch = p.search(egraph);
                                    let pmatch2 = p.search_eclass(egraph, pmatch.id);
                                            for m in matches{
                                            merge(&mut m.subst, pmatch.subst)
                                    }}
                                    */

                }


            }
            matches
        }
    */
    fn vars(&self) -> Vec<egg::Var> {
        let mut pats: Vec<_> = self
            .patterns
            .iter()
            .flat_map(
                |p| p.vars(), /*  match p {
                                      Bare(p) => p.vars().into_iter().chain(vec![].into_iter()), // this is unfortunate
                                      Eq(p1,p2) => p1.vars().into_iter().chain( p2.vars().into_iter())
                                  }
                              */
            )
            .collect();
        pats.sort();
        pats.dedup();
        pats
    }
}
/*
struct EqApplier {
    l : Pattern,
    r : Pattern
}

impl Applier for EqApplier {
    fn apply_one {

    }
}


struct IgnoreApply(Pattern)
// apply_one just inserts and ignores.

*/
fn run_file(file: Vec<Entry>) -> String {
    let mut queries = vec![];
    let mut rules = vec![];
    let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    for entry in file {
        match entry {
            Clause(Bare(head), body) => {
                let body = body
                    .iter()
                    .filter_map(|eqt| match eqt {
                        Bare(p) => Some(pattern_of_term(p)),
                        Eq(a, b) => None, // We need to deal with this case by both extending multipatterns and seperating in ConditionalEqual checks.
                    })
                    .collect();
                // let body = body.iter().map(|p| pattern_of_term(p)).collect();
                let searcher = MultiPattern { patterns: body };
                let applier = pattern_of_term(&head); // This is wrong because it'll unify the head. Sigh.
               /*  match head {
                     Bare(head) =>,
                     EqWrap(head) => | 
                 } */
                //rules.push(egg::Rewrite::new("", searcher, applier).unwrap());
            }
            Clause(Eq(a,b), body) => (),
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
