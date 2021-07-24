use egg::*;
use std::fmt::Write;

mod types;
pub use types::*;
use Term::*;
use Entry::*;
use EqWrap::*;

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




fn run_file(file: Vec<Entry>) -> String {
    let mut queries = vec![];
    let mut rules = vec![];
    let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    for entry in file {
        match entry {
            Clause(_head, _body) => (), /*{
            if body.len() == 0 {
            match head {
            Bare(a) => {
            is_ground(&a).map(|gt| eid_of_groundterm(&mut egraph, &gt));
            }
            Eq(a, b) => {
            if let Some(a) = is_ground(&a) {
            if let Some(b) = is_ground(&b) {
            let a_id = eid_of_groundterm(&mut egraph, &a);
            let b_id = eid_of_groundterm(&mut egraph, &b);
            egraph.union(a_id, b_id);
            }
            }
            }
            }
            } else {
            }
            } */
            Query(mut qs) => queries.append(&mut qs),
            Directive(_d) => (),
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
                rules.push(egg::Rewrite::new("", "", b, applier).unwrap());
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
        Err(e) => e
    }
}