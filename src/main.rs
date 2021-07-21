use std::fs;
extern crate nom;
use egg::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n, take_until},
    character::complete::{alphanumeric0, alphanumeric1, char, multispace0, satisfy},
    combinator::{map, map_res, opt, value},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum Term {
    Var(String),
    Apply(String, Vec<Term>),
}
// toplevel of term is eq only
#[derive(Debug)]
enum EqWrap<T> {
    Eq(T, T),
    Bare(T),
}
// There is an argument to be made that I should directly be using RecExpr for groundterm
// and 
/* enum EqGround {
    Eq(GroundTerm, GroundTerm),
    JustTerm(GroundTerm),
} */
use EqWrap::*;
use Term::*;
#[derive(Debug)]
enum Entry {
    Clause(EqWrap<Term>, Vec<EqWrap<Term>>),
    Fact(EqWrap<GroundTerm>),
    Rewrite(Term,Term, Vec<EqWrap<Term>>),
    Directive(Term),
    Query(Vec<EqWrap<Term>>),
}
use Entry::*;

/*struct Clause {
    head : EqTerm,
    body : Vec<EqTerm>
} */

fn clause(input: &str) -> IResult<&str, Entry> {
    let (input, head) = eqterm(input)?;
    let (input, body) = terminated(
        opt(preceded(tag(":-"), separated_list1(char(','), eqterm))),
        char('.'),
    )(input)?;
    let body = body.unwrap_or(vec![]);
    Ok((input, Clause(head, body)))
}

fn rewrite(input: &str) -> IResult<&str, Entry> {
    let (input, a) = term(input)?;
    let (input, head) = tag("<-")(input)?;
    let (input, b) = term(input)?;
    let (input, body) = terminated(separated_list0(char(','), eqterm),char('.'))(input)?;
    Ok((input, Rewrite(a,b, body)))
}

fn query(input: &str) -> IResult<&str, Entry> {
    map(
        delimited(tag("?-"), separated_list1(char(','), eqterm), char('.')),
        |eqterms| Query(eqterms),
    )(input)
}

fn directive(input: &str) -> IResult<&str, Entry> {
    map(delimited(tag(":-"), term, char('.')), |term| {
        Directive(term)
    })(input)
}

fn entry(input: &str) -> IResult<&str, Entry> {
    alt((query, directive, rewrite, clause))(input)
}

pub fn pinline_comment<'a>(i: &'a str) -> IResult<&'a str, ()> {
    value(
      (), // Output is thrown away.
      tuple((
        tag("/*"),
        take_until("*/"),
        tag("*/")
      ))
    )(i)
  }

fn file(input: &str) -> IResult<&str, Vec<Entry>> {
    many0(terminated(entry, opt(pinline_comment)))(input)
}

fn upper(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_uppercase())(input)
}

fn var(input: &str) -> IResult<&str, Term> {
    let (input, (c, s)) = pair(upper, alphanumeric0)(input)?;
    Ok((input, Var(format!("{}{}", c, s)))) // seems awkward. whatever
}

fn groundterm(input: &str) -> IResult<&str, GroundTerm> {
    let (input, head) = alphanumeric1(input)?;
    let (input, body) = opt(delimited(
        char('('),
        separated_list0(char(','), groundterm), // TODO: whitespace
        char(')'),
    ))(input)?;
    let body = body.unwrap_or(vec![]);
    Ok((input, GroundTerm{head : head.to_string(), args : body}))
}

fn eqgroundterm(input: &str) -> IResult<&str, EqWrap<GroundTerm>> {
    map(
        pair(groundterm, opt(preceded(char('='), groundterm))),
        |(t, ot)| match ot {
            Some(t2) => Eq(t, t2),
            None => Bare(t),
        },
    )(input)
}

fn apply(input: &str) -> IResult<&str, Term> {
    let (input, head) = alphanumeric1(input)?;
    let (input, body) = opt(delimited(
        char('('),
        separated_list0(char(','), term), // TODO: whitespace
        char(')'),
    ))(input)?;
    let body = body.unwrap_or(vec![]);
    Ok((input, Apply(head.to_string(), body)))
}
// Behaves incorrectly on capital named terms. Whatever. Don't do that.
fn term(input: &str) -> IResult<&str, Term> {
    alt((var, apply))(input)
}

fn eqterm(input: &str) -> IResult<&str, EqWrap<Term>> {
    map(
        pair(term, opt(preceded(char('='), term))),
        |(t, ot)| match ot {
            Some(t2) => Eq(t, t2),
            None => Bare(t),
        },
    )(input)
}



#[derive(Debug)]
struct GroundTerm {
    head: String,
    args: Vec<GroundTerm>,
}

fn is_ground(t: &Term) -> Option<GroundTerm> {
    match t {
        Var(_) => None,
        Apply(f, args) => {
            let oargs: Option<Vec<GroundTerm>> = args.iter().map(is_ground).collect();
            oargs.map(|args| GroundTerm {
                head: f.to_string(),
                args,
            })
        }
    }
}

fn eid_of_groundterm(egraph: &mut EGraph<SymbolLang, ()>, t: &GroundTerm) -> Id {
    let args = t.args.iter().map(|a| eid_of_groundterm(egraph, a)).collect();
    egraph.add(SymbolLang::new(t.head.clone(), args))
}

fn recexpr_of_groundterm_aux(expr: &mut RecExpr<SymbolLang>, t : &GroundTerm) -> Id {
    let expr_args = t.args.iter().map(| a| recexpr_of_groundterm_aux(expr, &a)).collect();
    expr.add(SymbolLang::new(t.head.clone(), expr_args ))
}
fn recexpr_of_groundterm(t : &GroundTerm) -> Id {
    let mut expr = RecExpr::default();
    recexpr_of_groundterm_aux(&mut expr,t)
}
/*
fn pattern_of_term(t : &Term) -> Pattern<SymbolLang> {
    let mut ast = RecExpr::default();
    fn worker(t : &Term){
        match t {
            Var(x) => ast.add(ENodeOrVar::Var(Var(Symbol::from("x"))))
            Apply(f,args) =>
              let args = args.iter().map(worker).collect();
             ast.add(ENodeOrVar::ENode( SymbolLang::new(f.clone(),args)))
        }
    }
    worker(t);
    let program = egg::machine::Program::compile_from_pat(&ast);
    Pattern { ast, program }
}
*/
fn sexp_of_term(t : &Term) -> String {
    match t {
        Var(x) => format!(" ?{} ",x),
        Apply(f,args) => {
            let args : String = args.iter().map(sexp_of_term).collect();
            format!("({}{})", f, args)
        }

    }
}

/*
fn merge_subst( &mut s1 : Subst, &mut s2 : Subst ) -> {
    for (v,i) in s2.vec {
        if let Some(id) = s1.insert(v,i){
            return true;
        }
    }
    return false;
}
*/
/*
// Private. options ; 
// 1 trasmute the memory. Yikes.
// 2 Rebuild the machine infrastructure in a file here. Compile a single machine that produces a single subst.
// 3 Fork egg
fn merge_subst( s1 : &Subst, s2 : &Subst ) -> Option<Subst>{
    let s1 = s1.clone();
    for (v,i) in s2.vec {
        if let Some(id) = s1.insert(v,i){
            return None;
        }
    }
    return Some(s1);
}
*/

// This sort of stuff is what From traits are for right?
fn pattern_of_term(t : &Term) -> Pattern<SymbolLang> {
    sexp_of_term(t).parse().unwrap()
}

fn run_file(file: Vec<Entry>) {
    let mut queries = vec![];
    let mut rules = vec![];
    let mut egraph: EGraph<SymbolLang, ()> = Default::default();
    for entry in file {
        match entry {
            Fact(f) => {


            }
            Clause(head, body) => {
                if body.len() == 0 {
                    match head {
                        Bare(a) => {
                            is_ground(&a).map(|gt| eid_of_groundterm(&mut egraph, &gt));
                        }
                        Eq(a, b) => if let Some(a) = is_ground(&a) {
                              if let Some(b) = is_ground(&b)  {
                                    let a_id = eid_of_groundterm(&mut egraph, &a);
                                    let b_id = eid_of_groundterm(&mut egraph, &b);
                                    egraph.union(a_id, b_id);
                                }
                            }
                        }
                    
                } else{


                }
            },
            Query(mut qs) => queries.append(&mut qs),
            Directive(d) => (),
            Rewrite(a,b,body) => (),
            Fact(x) => ()
        }
    }

    let mut runner = Runner::default().with_egraph(egraph).run(rules);
    runner.print_report();
    for q in queries{
        match q {
            Bare(a) => if let Some(a) = is_ground(&a) {
                eid_of_groundterm(&mut runner.egraph, &a);
            }
            Eq(a,b) => if let Some(a) = is_ground(&a) {
                if let Some(b) = is_ground(&b) {
                let aid = eid_of_groundterm(&mut runner.egraph, &a);
                let bid = eid_of_groundterm(&mut runner.egraph, &b);
                println!("{} = {}", aid, bid);
            }
        }
        }
        
    }


}

fn main() {
    println!("Hello, world!");
    dbg!(Var("fred".to_string()));
    dbg!(term("f()"));
    dbg!(clause("f()."));
    dbg!(clause("f():-f()."));
    dbg!(clause("f():-f(),greg()."));
    dbg!(clause("f():-f(),,greg()."));
    dbg!(clause("f(x)."));
    dbg!(clause("Xy."));
    dbg!(eqterm("f=g"));
    let filename = "example.pl";
    let mut contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents.retain(|c| !c.is_whitespace());
    match file(&contents){
        Err(e) => {dbg!(e);},
        Ok((rem,file)) => {
            dbg!(rem);
            dbg!(&file);
            run_file(file);

        }
    }
    
        dbg!(term("f(x,X,g(y))").map(|(_, t) | pattern_of_term(&t).to_string()));
        dbg!(term("f(r(HENRY,phanto),X,g(y))").map(|(_, t) | pattern_of_term(&t).to_string()));
        //dbg!(term("f(e(HENRY,phanto),X,g(y))"));

}
