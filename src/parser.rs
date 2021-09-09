use crate::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alphanumeric0, alphanumeric1, char, multispace0, multispace1, satisfy},
    combinator::{map, opt, value},
    error::ParseError,
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

fn clause(input: &str) -> IResult<&str, Entry> {
    let (input, head) = separated_list1(ws(char(',')), eqterm)(input)?;
    let (input, body) = preceded(ws(tag(":-")), separated_list1(ws(char(',')), eqterm))(input)?;
    Ok((input, Clause(head, body)))
}

fn rewrite(input: &str) -> IResult<&str, Entry> {
    let (input, a) = terminated(term, ws(tag("<-")))(input)?;
    let (input, b) = term(input)?;
    let (input, body) = opt(preceded(
        ws(tag(",")),
        separated_list1(ws(char(',')), eqterm),
    ))(input)?;
    let body = match body {
        None => vec![],
        Some(v) => v,
    };
    Ok((input, Rewrite(a, b, body)))
}

fn birewrite(input: &str) -> IResult<&str, Entry> {
    let (input, a) = terminated(term, ws(tag("<->")))(input)?;
    let (input, b) = term(input)?;
    Ok((input, BiRewrite(a, b)))
}

fn query(input: &str) -> IResult<&str, Entry> {
    map(
        preceded(ws(tag("?-")), separated_list1(ws(char(',')), eqterm)),
        |eqterms| Query(eqterms),
    )(input)
}
/*
fn forall(input: &str) -> IResult<&str, Formula> {
    let (input, v) = preceded(tag("forall("), alphanumeric0)(input)?;
    let (input, f) = terminated(formula, tag(")"))(input)?;
    Ok((input, Formula::ForAll(vec![v.to_string()], Box::new(f))))
}

fn conj(input: &str) -> IResult<&str, Formula> {
    map(separated_list1(alt((tag(","), tag("&"))), formula), |fs| {
        Formula::Conj(fs)
    })(input)
}

fn atom(input: &str) -> IResult<&str, Formula> {
    map(eqterm, |eqt| Formula::Atom(eqt))(input)
}

fn formula(input: &str) -> IResult<&str, Formula> {
    alt((
        delimited(tag("("), formula, tag(")")),
        alt((forall, conj, atom)),
    ))(input)
}
*/
fn include(input: &str) -> IResult<&str, Directive> {
    map(
        delimited(ws(tag("include(")), take_until(")"), ws(tag(")"))),
        |filename: &str| Directive::Include(filename.to_string()),
    )(input)
}

fn directive(input: &str) -> IResult<&str, Entry> {
    map(preceded(ws(tag(":-")), include), |d| Directive(d))(input)
}

fn fact(input: &str) -> IResult<&str, Entry> {
    map(eqgroundterm, |a| Fact(a))(input)
}

fn entry(input: &str) -> IResult<&str, Entry> {
    // I should factor this more.
    ws(terminated(
        alt((
            query, directive, axiom, goal, birewrite, rewrite, clause, fact,
        )),
        char('.'),
    ))(input)
}

pub fn pinline_comment<'a>(i: &'a str) -> IResult<&'a str, ()> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
    )(i)
}

fn file(input: &str) -> IResult<&str, Vec<Entry>> {
    let (input, _) = many0(ws(pinline_comment))(input)?;
    many0(terminated(entry, many0(ws(pinline_comment))))(input)
    //many0(alt((entry, map(pinline_comment, )))(input)
}

pub fn parse_file(input: String) -> Result<Vec<Entry>, String> {
    // input.retain(|c| !c.is_whitespace());
    match file(&input) {
        Ok((rem, f)) => {
            if rem.is_empty() {
                Ok(f)
            } else {
                Err(format!("Remainder: {}", rem))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
fn upper(input: &str) -> IResult<&str, char> {
    satisfy(|c| c.is_ascii_uppercase())(input)
}

fn var(input: &str) -> IResult<&str, Term> {
    let (input, (c, s)) = pair(upper, alphanumeric0)(input)?;
    Ok((input, Var(format!("{}{}", c, s)))) // seems awkward. whatever
}
// TODO: infix operators.
fn groundterm(input: &str) -> IResult<&str, GroundTerm> {
    let (input, head) = alphanumeric1(input)?;
    let (input, body) = opt(delimited(
        ws(char('(')),
        separated_list0(ws(char(',')), groundterm), // TODO: whitespace
        ws(char(')')),
    ))(input)?;
    let body = body.unwrap_or(vec![]);
    Ok((
        input,
        GroundTerm {
            head: head.to_string(),
            args: body,
        },
    ))
}

fn eqgroundterm(input: &str) -> IResult<&str, EqWrap<GroundTerm>> {
    map(
        pair(groundterm, opt(preceded(ws(char('=')), groundterm))),
        |(t, ot)| match ot {
            Some(t2) => Eq(t, t2),
            None => Bare(t),
        },
    )(input)
}

fn apply(input: &str) -> IResult<&str, Term> {
    let (input, head) = alphanumeric1(input)?;
    let (input, body) = opt(delimited(
        ws(char('(')),
        separated_list0(ws(char(',')), term), // TODO: whitespace
        ws(char(')')),
    ))(input)?;
    let body = body.unwrap_or(vec![]);
    Ok((input, Apply(head.to_string(), body)))
}
// Behaves incorrectly on capital named terms. Whatever. Don't do that.
fn term(input: &str) -> IResult<&str, Term> {
    ws(alt((var, apply)))(input)
}

fn eqterm(input: &str) -> IResult<&str, EqWrap<Term>> {
    map(
        pair(term, opt(preceded(ws(char('=')), term))),
        |(t, ot)| match ot {
            Some(t2) => Eq(t, t2),
            None => Bare(t),
        },
    )(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/*
I could redesign it so that variables must be bound.

*/

//existsunique

// Hmmm. Should the parens go... somewhere deeper?
// What about f (f x).
fn primform(input: &str) -> IResult<&str, Formula> {
    alt((delimited(tag("("), form, tag(")")), atom2))(input)
}

fn conjform(input: &str) -> IResult<&str, Formula> {
    map(separated_list1(ws(tag("/\\")), primform), |mut fs| {
        if fs.len() == 1 {
            fs.remove(0)
        } else {
            Formula::Conj(fs)
        }
    })(input)
}
fn disjform(input: &str) -> IResult<&str, Formula> {
    map(separated_list1(ws(tag("\\/")), conjform), |mut fs| {
        if fs.len() == 1 {
            fs.remove(0)
        } else {
            Formula::Disj(fs)
        }
    })(input)
}

fn implform(input: &str) -> IResult<&str, Formula> {
    map(separated_list1(ws(tag("=>")), disjform), |fs| {
        let mut iter = fs.into_iter().rev();
        let mut f = iter.next().unwrap(); // since seperatedlist1 we can just unwrap
        for f1 in iter {
            f = Formula::Impl(Box::new(f1), Box::new(f));
        }
        f
    })(input)
}

fn quantifier(input: &str) -> IResult<&str, Formula> {
    let (input, q) = alt((
        value(
            Formula::ForAll as fn(Vec<String>, Box<Formula>) -> Formula,
            tag("forall"),
        ),
        // fn(_,_) -> _ also works
        //more cryptic or not? Function pointer casting https://stackoverflow.com/questions/27895946/expected-fn-item-found-a-different-fn-item-when-working-with-function-pointer
        value(
            Formula::Exists as fn(Vec<String>, Box<Formula>) -> Formula,
            tag("exists"),
        ),
    ))(input)?;
    let (input, args) =
        terminated(ws(separated_list1(multispace1, alphanumeric1)), tag(","))(input)?;
    let (input, f) = form(input)?;
    Ok((
        input,
        q(args.iter().map(|s| s.to_string()).collect(), Box::new(f)),
    ))
}

fn form(input: &str) -> IResult<&str, Formula> {
    ws(alt((quantifier, implform)))(input)
}

/*

forall x,  => yada.
forall x, => yada.

-------------------- `--` starts a comment line?
|- something.   thi is query syntax
|- something.

*/

fn primterm(input: &str) -> IResult<&str, Term> {
    alt((
        delimited(tag("("), term2, tag(")")),
        map(alphanumeric1, |s: &str| Apply(s.to_string(), vec![])),
    ))(input)
}
fn apply2(input: &str) -> IResult<&str, Term> {
    let (input, head) = terminated(alphanumeric1, multispace0)(input)?;
    let (input, body) = separated_list0(multispace1, primterm)(input)?;
    Ok((input, Apply(head.to_string(), body)))
}
// SHould just switch to groundterm
fn term2(input: &str) -> IResult<&str, Term> {
    ws(alt((delimited(tag("("), term2, tag(")")), apply2)))(input)
}

fn eqterm2(input: &str) -> IResult<&str, EqWrap<Term>> {
    map(
        ws(pair(term2, opt(preceded(ws(char('=')), term2)))),
        |(t, ot)| match ot {
            Some(t2) => Eq(t, t2),
            None => Bare(t),
        },
    )(input)
}

fn atom2(input: &str) -> IResult<&str, Formula> {
    map(eqterm2, |eqt| Formula::Atom(eqt))(input)
}

fn axiom(input: &str) -> IResult<&str, Entry> {
    let (input, name) = delimited(ws(tag("Axiom")), alphanumeric1, ws(tag(":")))(input)?;
    let (input, f) = ws(form)(input)?;
    Ok((input, Axiom(name.to_string(), f)))
}

fn goal(input: &str) -> IResult<&str, Entry> {
    map(preceded(ws(tag("|-")), form), Goal)(input)
}
/*
fn entry2(input: &str) -> IResult<&str, Mode> {
    // I should factor this more.
    terminated(alt((goal, map(form, Mode::Axiom))), char('.'))(input)
}

fn helper(input: &str) -> IResult<&str, Vec<Mode>> {
    let (input, _) = many0(ws(pinline_comment))(&input)?;
    ws(many0(terminated(entry2, many0(ws(pinline_comment)))))(input)
}

pub fn parse_file2(mut input: String) -> Result<Vec<Mode>, String> {
    match helper(&input) {
        Ok((rem, f)) => {
            if rem.is_empty() {
                Ok(f)
            } else {
                Err(format!("Remainder: {}", rem))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
*/
//https://coq.inria.fr/refman/language/coq-library.html
// ->  <->  \/ /\ ~ =
//alt((eqterm, conj, disj, implies, forall, exists ))

#[cfg(test)]
mod tests {
    use super::*;
    use Formula::*;
    #[test]
    fn parser2_test() {
        let f = "f".to_string();
        let x = Apply("x".to_string(), vec![]);
        let fx = Atom(Bare(Apply(f.clone(), vec![x.clone()])));

        assert_eq!(term2("(f x)").unwrap().1, Apply(f.clone(), vec![x.clone()]));

        assert_eq!(
            term2(" f x  ").unwrap().1,
            Apply(f.clone(), vec![x.clone()])
        );
        assert_eq!(
            eqterm2(" f x  ").unwrap().1,
            Bare(Apply(f.clone(), vec![x.clone()]))
        );
        assert_eq!(atom2(" f x  ").unwrap().1, fx);
        assert!(quantifier(" f x  ").is_err());
        //assert!(conj2(" f x  ").is_err());

        assert_eq!(form(" f x  ").unwrap().1, fx);
        assert_eq!(form(" f x  ").unwrap().1, fx);
        assert_eq!(
            form(" f x  /\\ f x").unwrap().1,
            Conj(vec![fx.clone(), fx.clone()])
        );
        assert_eq!(
            form("f x  /\\ f x\\/f x").unwrap().1,
            Disj(vec![Conj(vec![fx.clone(), fx.clone()]), fx.clone()])
        );

        assert_eq!(
            form("forall x, f x").unwrap().1,
            ForAll(vec!["x".to_string()], Box::new(fx.clone()))
        );
        assert_eq!(
            form("forall x, f x => f x").unwrap().1,
            ForAll(
                vec!["x".to_string()],
                Box::new(Impl(Box::new(fx.clone()), Box::new(fx.clone())))
            )
        );

        assert_eq!(
            form("forall x y z, f x").unwrap().1,
            ForAll(
                vec!["x".to_string(), "y".to_string(), "z".to_string()],
                Box::new(fx.clone())
            )
        );

        //assert_eq!(formula2("(f x)").unwrap().1,  Atom( Bare(Apply(f, vec![x]) ) ));
    }
    #[test]
    fn it_works() {
        let f = "f".to_string();
        let x = Apply("x".to_string(), vec![]);
        assert_eq!(term("f()").unwrap().1, Apply("f".into(), vec![]));
        assert_eq!(
            entry("f().").unwrap().1,
            Fact(Bare(GroundTerm {
                head: f,
                args: vec![]
            }))
        );
        assert_eq!(entry("x<->x.").unwrap().1, BiRewrite(x.clone(), x));
        /* (clause("f()."));
        dbg!(clause("f():-f()."));
        dbg!(clause("f():-f(),greg()."));
        dbg!(clause("f():-f(),,greg()."));
        dbg!(clause("f(x)."));
        dbg!(clause("Xy."));
        dbg!(eqterm("f=g")); */
    }
    #[test]
    fn includetest() {
        let f = "foo.pl".to_string();
        assert_eq!(
            entry(":-include(foo.pl).").unwrap().1,
            Directive(Directive::Include(f.clone()))
        );
    }
}

/*
TODO: make these tests
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
            dbg!(term("f(x,X,g(y))").map(|(_, t) | pattern_of_term(&t).to_string()));
        dbg!(term("f(r(HENRY,phanto),X,g(y))").map(|(_, t) | pattern_of_term(&t).to_string()));
        //dbg!(term("f(e(HENRY,phanto),X,g(y))"));
*/
