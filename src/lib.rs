use egg::*;
use std::fmt::Write;

mod types;
pub use types::*;
use Entry::*;
use EqWrap::*;
use Term::*;
//use types::Directive::*;
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

fn merge_substs(substs1: &Vec<Subst>, substs2: &Vec<Subst>) -> Vec<Subst> {
    // s1s.iter()
    //    .flat_map(|s1| s2s.iter().filter_map(move |s2| merge_subst2(s1, s2)))
    //    .collect()
    let mut substs = vec![]; // this is merge substs above.
    for subst1 in substs1 {
        for subst2 in substs2 {
            if let Some(subst) = merge_subst2(subst1, subst2) {
                substs.push(subst);
            }
        }
    }
    substs
}

#[derive(Debug, PartialEq)]
struct MultiPattern<S> {
    patterns: Vec<S>,
}
use std::fmt;

impl<S> fmt::Display for MultiPattern<S>
where
    S: fmt::Display,
{
    fn fmt(&self, buf: &mut fmt::Formatter) -> fmt::Result {
        let mut iter = self.patterns.iter();
        if let Some(item) = iter.next() {
            write!(buf, "{}", item)?;
            for item in iter {
                write!(buf, ", {}", item)?;
            }
        }
        Ok(())
    }
}

impl<L: Language, A: Analysis<L>> Searcher<L, A> for EqWrap<Pattern<L>> {
    fn search_eclass(&self, egraph: &EGraph<L, A>, eclass: Id) -> Option<SearchMatches> {
        match self {
            Bare(p) => p.search_eclass(egraph, eclass),
            Eq(p1, p2) => {
                let matches = p1.search_eclass(egraph, eclass)?;
                let matches2 = p2.search_eclass(egraph, eclass)?;
                let substs = merge_substs(&matches.substs, &matches2.substs);
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

impl<L: Language, A: Analysis<L>, P: Searcher<L, A>> Searcher<L, A> for MultiPattern<P> {
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
            .flat_map(|p| <Searcher<L, A>>::vars(p))
            .collect();
        pats.sort();
        pats.dedup();
        pats
    }
}

// Hmm. Should I dfefine an applier for EqWrap<Pattern> instead?
impl<N, L, A> Applier<L, N> for MultiPattern<A>
where
    L: Language,
    N: Analysis<L>,
    A: Applier<L, N>,
{
    fn apply_one(&self, egraph: &mut EGraph<L, N>, eclass: Id, subst: &Subst) -> Vec<Id> {
        let mut added = vec![]; // added are union updates, of which there are none
        for applier in &self.patterns {
            added.extend(applier.apply_one(egraph, eclass, subst));
        }
        added
    }

    fn apply_matches(&self, egraph: &mut EGraph<L, N>, matches: &[SearchMatches]) -> Vec<Id> {
        let mut added = vec![];
        for applier in &self.patterns {
            added.extend(applier.apply_matches(egraph, matches));
        }
        added
    }

    fn vars(&self) -> Vec<egg::Var> {
        let mut vars = vec![];
        for applier in &self.patterns {
            vars.extend(applier.vars());
        }
        // Is this necessary? How is var even used?
        vars.sort();
        vars.dedup();
        vars
    }
}

impl<N, L, A> Applier<L, N> for EqWrap<A>
where
    L: Language,
    N: Analysis<L>,
    A: Applier<L, N>,
{
    fn apply_one(&self, _egraph: &mut EGraph<L, N>, _eclass: Id, _subst: &Subst) -> Vec<Id> {
        // self.0.apply_one(egraph, eclass, subst)
        panic!("EqApply.apply_one was called");
    }

    // Could copy using apply_pat for better efficiency
    fn apply_matches(&self, egraph: &mut EGraph<L, N>, matches: &[SearchMatches]) -> Vec<Id> {
        match self {
            Bare(a) => a.apply_matches(egraph, matches),
            Eq(l, r) => {
                let mut added = vec![]; // added are union updates, of which there are none
                for mat in matches {
                    for subst in &mat.substs {
                        // This should be ok because we know they are patterns. Not very safe.
                        let id1 = l.apply_one(egraph, 0.into(), subst)[0];
                        let id2 = r.apply_one(egraph, 0.into(), subst)[0];
                        let (to, did_something) = egraph.union(id1, id2);
                        if did_something {
                            added.push(to)
                        }
                    }
                }
                added
            }
        }
    }

    fn vars(&self) -> Vec<egg::Var> {
        match self {
            Bare(a) => a.vars(),
            Eq(l, r) => {
                let mut vars = l.vars();
                vars.extend(r.vars());
                vars
            }
        }
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
    fn apply_one(&self, _egraph: &mut EGraph<L, N>, _eclass: Id, _subst: &Subst) -> Vec<Id> {
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
-[x] MultiApplier could be useful / efficient
-[] Using Conditional Equals could be useful if all variables known
-[] But really getting patterns to compile with substituion pieces considered known subsumes this optimization I think in modern egg with yihong's optimization.
-[] Sanity checks that needed variables exist would be good. It does crash with named rules, so that's something.
-[] May want to run Runner multiple times since it may not get restarted. Currently I have that vec![0] hack
-[] _ for dummy variables
-[] The ability to check to see if something is in the egraph.
-[] graphviz dumping the egraph
-[] harrop formula
-[] merge_subst that doesn't copy?
-[] Give rules names. Keep a hash table of them?
-[x] Queries with variables
-[x] Queries should be conjunctions
-[X] a REPL would be sweet. especially if we have higher order rules, we could watch the database, add queries
-[] termination based on the query condition
-[] side effectful searchers and appliers (printing mostly), functions.
-[] Astsize with weighting? Does that get me anywhere?
-[] infix operators
-[] better printers
-[] rewrite/proof files that allow intermediate queriess. set of support?
-[x] cli
-[] smtlib subset (forall (a b ) (= (f a) (g c))  ) ! :pattern) or horn cluase style.
-[] vaguely ML/coq style synax
-[] tptp syntax?
-[] push pop directives instead of clear.
-[] only allow stuff that compresses the egraph? Appliers that do not add terms to the egraph or only add a couple? Or keeps counts.
-[] directives to changes egraph params. or flags?
-[] Macros/simplification stage?
-[] typed symbollang - would this even be an optimization?
*/

/*



run_file
repl() {
    loop{
        receive_string
        process
    }
}
:- clear.
: halt.
:- [yada.pl].
*/

type SymExpr = RecExpr<SymbolLang>;
type SymEGraph = EGraph<SymbolLang, ()>;

fn simplify(egraph: &SymEGraph, eid: Id) -> SymExpr {
    let extractor = Extractor::new(egraph, AstSize);
    let (_best_cost, best) = extractor.find_best(eid);
    best
}

fn print_subst<T: std::fmt::Write>(
    buf: &mut T,
    egraph: &EGraph<SymbolLang, ()>,
    subst: &Subst,
) -> Result<(), std::fmt::Error> {
    write!(buf, "[");
    let mut iter = subst.vec.iter();
    if let Some((k, eid)) = iter.next() {
        let best = simplify(egraph, *eid);
        write!(buf, "{} = {}", k, best)?;
        for (k, eid) in iter {
            let best = simplify(egraph, *eid);
            write!(buf, ", {} = {}", k, best)?;
        }
    }
    write!(buf, "]")
}

type SymMultiPattern = MultiPattern<EqWrap<Pattern<SymbolLang>>>;

// Current directory and already included set?
#[derive(Debug)]
pub struct Env {
    runner: Runner<SymbolLang, ()>,
    rules: Vec<egg::Rewrite<SymbolLang, ()>>,
    queries: Vec<MultiPattern<EqWrap<Pattern<SymbolLang>>>>,
}

impl Default for Env {
    fn default() -> Self {
        Env {
            runner: Runner::default(),
            queries: vec![],
            rules: vec![],
        }
    }
}

use std::fs;
fn load_file(env: &mut Env, filename: &str) -> Result<(), String> {
    match fs::read_to_string(filename) {
        Err(e) => Err(format!("Error: file {} not found", filename)),
        Ok(contents) => match parse_file(contents) {
            Err(e) => Err(format!(
                "Error : file {} failed to parse with error : {}",
                filename, e
            )),
            Ok(entries) => {
                for entry in entries {
                    process_entry(env, entry);
                }
                Ok(())
            }
        },
    }
}
// impl Env?
pub fn process_entry(state: &mut Env, entry: Entry) {
    let queries = &mut state.queries;
    let rules = &mut state.rules;
    let mut egraph = &mut state.runner.egraph;
    match entry {
        Directive(types::Directive::Include(filename)) => load_file(state, &filename).unwrap(), // TODO: This include is not relative. That's not good.
        Fact(Eq(a, b)) => {
            let a_id = eid_of_groundterm(&mut egraph, &a);
            let b_id = eid_of_groundterm(&mut egraph, &b);
            egraph.union(a_id, b_id);
        }
        Fact(Bare(a)) => {
            eid_of_groundterm(&mut egraph, &a);
        }
        Clause(head, body) => {
            let body = body
                .iter()
                .map(|eqt| match eqt {
                    Bare(p) => Bare(pattern_of_term(p)),
                    Eq(a, b) => Eq(pattern_of_term(a), pattern_of_term(b)),
                })
                .collect();
            let searcher = MultiPattern { patterns: body };
            let head = head
                .iter()
                .map(|eqt| match eqt {
                    Bare(p) => Bare(pattern_of_term(p)),
                    Eq(a, b) => Eq(pattern_of_term(a), pattern_of_term(b)),
                })
                .collect();
            let applier = MultiPattern { patterns: head };
            rules.push(
                egg::Rewrite::new(format!("{}:-{}.", applier, searcher), searcher, applier)
                    .unwrap(),
            );
            /* // consider as a small optimization for Single headed clauses (which are by far the most common.)
            match head {
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
            }; */
        }
        BiRewrite(a, b) => {
            let a = pattern_of_term(&a);
            let b = pattern_of_term(&b);
            rules.push(egg::Rewrite::new(format!("{}->{}", a,b) , a.clone(), b.clone()).unwrap());
            rules.push(egg::Rewrite::new(format!("{}->{}", b,a) , b, a).unwrap());
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
            let condition = move |egraph: &mut EGraph<_, ()>, eclass: Id, subst: &Subst| {
                conditions.iter().all(|c| c.check(egraph, eclass, subst))
            };
            let applier = ConditionalApplier { condition, applier };
            rules.push(egg::Rewrite::new(format!("{} -{:?}> {}",b,body,a), b, applier).unwrap());
        }
        Query(qs) => {
            let qs = qs
                .iter()
                .map(|eqt| match eqt {
                    // |eqt| eqt.map(|t| pattern_of_term(&t)) /*
                    Bare(p) => Bare(pattern_of_term(p)),
                    Eq(a, b) => Eq(pattern_of_term(a), pattern_of_term(b)),
                })
                .collect();
            queries.push(MultiPattern { patterns: qs });
        }
    }
}

// Refactor this to return not string.
fn run_file(file: Vec<Entry>) -> String {
    let mut env = Env::default();
    for entry in file {
        process_entry(&mut env, entry)
    }
    let runner = env.runner.run(&env.rules);
    runner.print_report();
    // runner.egraph.dot().to_png("target/foo.png").unwrap();
    let mut buf = String::new();
    for q in env.queries {
        writeln!(buf, "-? {}", q);
        let matches = q.search(&runner.egraph);
        if matches.iter().all(|mat| mat.substs.len() == 0) {
            // why are empty matches returned? Seems like a bug.
            writeln!(buf, "unknown.");
        } else {
            for mat in matches {
                for subst in mat.substs {
                    print_subst(&mut buf, &runner.egraph, &subst);
                    writeln!(buf, ";");
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
