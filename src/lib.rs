use egg::*;
use std::fmt::Write;

mod gensym;
mod logic;
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
/* // For use in the include directive
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
*/

use std::collections::HashSet;
struct Env2 {
    freshvars: HashSet<String>, // forall x adds into this set
    metavars: HashSet<String>,  // exists x add into this set.
                                // entries : Vec<Entry>,
}

impl Env2 {
    fn new() -> Self {
        Env2 {
            freshvars: HashSet::default(),
            metavars: HashSet::default(),
        }
    }
}

fn interp_term(env: &mut Env2, t: &Term) -> Term {
    match t {
        Var(x) => panic!("Impossible"), // should parse formula at groundterms.
        Apply(f, args) => {
            if args.len() == 0 && env.freshvars.contains(f) {
                Var(f.clone())
            } else {
                Apply(
                    f.to_string(),
                    args.iter().map(|f2| interp_term(env, f2)).collect(),
                )
            }
        }
    }
}

fn interp_eqwrap(env: &mut Env2, t: &EqWrap<Term>) -> EqWrap<Term> {
    match t {
        Eq(a, b) => Eq(interp_term(env, a), interp_term(env, b)),
        Bare(a) => Bare(interp_term(env, a)),
    }
}

fn interp_term_goal(env: &mut Env2, t: &Term) -> Term {
    match t {
        Var(x) => panic!("Impossible"),
        Apply(f, args) => {
            if args.len() == 0 && env.metavars.contains(f) {
                Var(f.clone())
            } else {
                Apply(
                    f.to_string(),
                    args.iter().map(|f2| interp_term_goal(env, f2)).collect(),
                )
            }
        }
    }
}

fn interp_eqwrap_goal(env: &mut Env2, t: &EqWrap<Term>) -> EqWrap<Term> {
    match t {
        Eq(a, b) => Eq(interp_term_goal(env, a), interp_term_goal(env, b)),
        Bare(a) => Bare(interp_term_goal(env, a)),
    }
}
use Formula::*;
// We shouldn't be using mutable envs. What am I thinking?


/*
More imperative style to a program?
enum SearchProgram {
    Run,
    Clear,   
}
*/

// Module?
pub struct Program {
    // eqfacts and facts, or just duplicate for base facts?
    facts : Vec<(RecExpr<SymbolLang>, RecExpr<SymbolLang>)>,
    rules : Vec<egg::Rewrite<SymbolLang, ()>>,
    queries : Vec<MultiPattern<EqWrap<Pattern<SymbolLang>>>>
}

impl Default for Program {
    fn default() -> Self {
        Program {
            facts: vec![],
            queries: vec![],
            rules: vec![],
        }
    }
}

pub fn process_entry_prog(prog: &mut Program, entry: Entry) {
    match entry {
        Directive(types::Directive::Include(filename)) => (), // load_file(state, &filename).unwrap(), 
        Fact(Eq(a, b)) => {
            let a = recexpr_of_groundterm(&a);
            let b = recexpr_of_groundterm(&b);
            prog.facts.push(  ( a,b )  )
        }
        Fact(Bare(a)) => {
            let a = recexpr_of_groundterm(&a);
            prog.facts.push(  ( a.clone(),a )  )
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
            prog.rules.push(
                egg::Rewrite::new(format!("{}:-{}.", applier, searcher), searcher, applier)
                    .unwrap(),
            );
        }
        BiRewrite(a, b) => {
            let a = pattern_of_term(&a);
            let b = pattern_of_term(&b);
            prog.rules.push(egg::Rewrite::new(format!("{}->{}", a, b), a.clone(), b.clone()).unwrap());
            prog.rules.push(egg::Rewrite::new(format!("{}->{}", b, a), b, a).unwrap());
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
            prog.rules.push(egg::Rewrite::new(format!("{} -{:?}> {}", b, body, a), b, applier).unwrap());
        }
        Query(qs) => {
            let qs = qs
                .iter()
                .map(pattern_of_eqterm)
                .collect();
            prog.queries.push(MultiPattern { patterns: qs });
        }
        Axiom(_name, f) => interp_formula(prog, &mut Env2::new(), f), // I should use the name
        Goal(f) => interp_goal(prog, &mut Env2::new(), f)
    }
}


fn run_program2(prog: &Program) -> Vec<Vec<Subst>> {
    let mut runner = Runner::default()
        .with_iter_limit(30)
        .with_node_limit(10_000)
        .with_time_limit(Duration::from_secs(5));
    let (runner, res) = run_program(prog, runner);
    res
}

fn run_program(prog: &Program, mut runner : Runner<SymbolLang,()>) -> (Runner<SymbolLang,()>, Vec<Vec<Subst>>)  {
    let egraph = &mut runner.egraph;
    for (a,b) in &prog.facts {
        let a_id = egraph.add_expr(&a); 
        let b_id = egraph.add_expr(&b);
        egraph.union(a_id, b_id);
    }
    let runner = runner.run(&prog.rules);
    let res = prog.queries.iter().map(|q| {
        let matches = q.search(&runner.egraph);
        matches.into_iter().flat_map(|mat|
            mat.substs
        ).collect()
    }).collect();
    (runner, res)
}

fn interp_goal(prog : &mut Program, env: &mut Env2, formula: Formula) {
    match formula {
        Conj(fs) => {
            let ps = fs.iter()
                .map(|g| match g {
                    Atom(g) => pattern_of_eqterm(&interp_eqwrap_goal(env, g)),
                    _ => panic!("unexpected form in goal"),
                })
                .collect();
            prog.queries.push( MultiPattern {patterns : ps} )
         }
        ,
        Atom(f) => {
            let g = MultiPattern {patterns : vec![pattern_of_eqterm(&interp_eqwrap_goal(env, &f))] };
            prog.queries.push(g)
        }
        Exists(vs, f) => {
            env.metavars.extend(vs);
            interp_goal(prog, env, *f)
        }
        /*
        Impl(hyp, conc) => {
            let hyp = interp_formula(env, hyp);
            let mut conc = interp_goal(env, conc);
            conc.push(hyp);
            conc
        }*/
        _ => panic!("no other goal"),
    }
}


// I should make env immutable this is not right as is.
// Or what is even the point of being this fancy
fn interp_formula(prog : &mut Program, env: &mut Env2, formula: Formula) {
    match formula {
        Atom(a) => {
            let e = match interp_eqwrap(env, &a) {
                Eq(a, b) => {
                    let a = recexpr_of_groundterm(&is_ground(&a).unwrap());
                    let b = recexpr_of_groundterm(&is_ground(&b).unwrap());
                    prog.facts.push( (a,b) )
                }
                Bare(a) => {
                    let a = recexpr_of_groundterm(&is_ground(&a).unwrap());
                    prog.facts.push( (a.clone(),a ))
                }
            };
            //Fact(e)
            
        }
        Impl(hyp, conc) => {
            let hyps = match *hyp {
                Atom(hyp) => vec![pattern_of_eqterm(&interp_eqwrap(env, &hyp))],
                Conj(hyps) => hyps
                    .iter()
                    .map(|hyp| match hyp {
                        Atom(hyp) => pattern_of_eqterm(&interp_eqwrap(env, hyp)),
                        _ => panic!("invalid hyp in conj"),
                    })
                    .collect(),
                _ => panic!("Invalid hyp {:?}", *hyp),
            };
            // I should be not duplicating code here.
            let concs = match *conc {
                Atom(conc) => vec![pattern_of_eqterm(&interp_eqwrap(env, &conc))],
                Conj(concs) => concs
                    .iter()
                    .map(|conc| match conc {
                        Atom(conc) => pattern_of_eqterm(&interp_eqwrap(env, conc)),
                        _ => panic!("invalid conc in conj"),
                    })
                    .collect(),
                _ => panic!("Invalid conc {:?}", *conc),
            };
            let searcher = MultiPattern { patterns: hyps };
            let applier = MultiPattern { patterns: concs };
            prog.rules.push( egg::Rewrite::new("", searcher, applier).unwrap() )
            // Clause(hyps, concs)
        }
        //Impl(box Atom(hyp), box Atom(conc)) => Clause(vec![interp_eqwrap(env, hyp)] , vec![interp_eqwrap(env, conc)),
        ForAll(vs, f) => {
            env.freshvars.extend(vs);
            interp_formula(prog, env, *f)
        }
        _ => panic!("unexpected formula {:?} in interp_formula", formula),
    }
}
/*

// allowing P /\ A => B in head of rules would be useful for exists_unique.


// This vs implementing searcher for a formula itself.
fn interp_searcher(formula : Formula) -> impl Searcher {
    match formula {
        Conj(xs) => ,
        Atom(Eq(a,b)) =>,
        Atom(Bare(a)) =>,
        _ => panic
    }

}

// Alt pattern implements "Or" search.
// It should run each of it's searchers and collate the results.
// Unlike MultiPattern it make no sense as an Applier, since we don't know which case to use.
struct AltPattern<P>{
    pats : Vec<P>
}

// Only succeeds if P fails.
struct NegPattern<P>{
    pat : P
}

*/


use core::time::Duration;
// Refactor this to return not string.
fn run_file(file: Vec<Entry>) -> String {
    //let mut env = Env::default();
    let mut prog = Program::default();

    for entry in file {
        //process_entry(&mut env, entry)
        process_entry_prog(&mut prog, entry)
    }
    let mut runner = Runner::default()
        .with_iter_limit(30)
        .with_node_limit(10_000)
        .with_time_limit(Duration::from_secs(5));
    let (runner, query_results) = run_program(&prog, runner);
    // Two useful things to turn on. Command line arguments?
    //runner.print_report();
    // runner.egraph.dot().to_png("target/foo.png").unwrap();
    let mut buf = String::new();
    for (q, res) in prog.queries.iter().zip(query_results) {
        writeln!(buf, "-? {}", q);
        //let matches = q.search(&runner.egraph);
        if res.len() == 0 {
            writeln!(buf, "unknown.");
        } else{
        for subst in res {
            print_subst(&mut buf, &runner.egraph, &subst);
            writeln!(buf, ";");
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
