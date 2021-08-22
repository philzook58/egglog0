use crate::*;
use logic::*;
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
    let (input, head) = separated_list1(char(','), eqterm)(input)?;
    let (input, body) = preceded(tag(":-"), separated_list1(char(','), eqterm))(input)?;
    Ok((input, Clause(head, body)))
}

fn rewrite(input: &str) -> IResult<&str, Entry> {
    let (input, a) = terminated(term, tag("<-"))(input)?;
    let (input, b) = term(input)?;
    let (input, body) = opt(preceded(tag(","), separated_list1(char(','), eqterm)))(input)?;
    let body = match body {
        None => vec![],
        Some(v) => v,
    };
    Ok((input, Rewrite(a, b, body)))
}

fn birewrite(input: &str) -> IResult<&str, Entry> {
    let (input, a) = terminated(term, tag("<->"))(input)?;
    let (input, b) = term(input)?;
    Ok((input, BiRewrite(a, b)))
}

fn query(input: &str) -> IResult<&str, Entry> {
    map(
        preceded(tag("?-"), separated_list1(char(','), eqterm)),
        |eqterms| Query(eqterms),
    )(input)
}

fn forall(input: &str) -> IResult<&str, Formula> {
    let (input, v) = preceded(tag("forall("), alphanumeric0)(input)?;
    let (input, f) = terminated(formula, tag(")"))(input)?;
    Ok((input, Formula::ForAll(v.to_string(), Box::new(f))))
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

fn include(input: &str) -> IResult<&str, Directive> {
    map(
        delimited(tag("include("), take_until(")"), tag(")")),
        |filename: &str| Directive::Include(filename.to_string()),
    )(input)
}

fn directive(input: &str) -> IResult<&str, Entry> {
    map(preceded(tag(":-"), include), |d| Directive(d))(input)
}

fn fact(input: &str) -> IResult<&str, Entry> {
    map(eqgroundterm, |a| Fact(a))(input)
}

fn entry(input: &str) -> IResult<&str, Entry> {
    // I should factor this more.
    terminated(
        alt((query, directive, birewrite, rewrite, clause, fact)),
        char('.'),
    )(input)
}

pub fn pinline_comment<'a>(i: &'a str) -> IResult<&'a str, ()> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
    )(i)
}

fn file(input: &str) -> IResult<&str, Vec<Entry>> {
    let (input, _) = many0(pinline_comment)(input)?;
    many0(terminated(entry, many0(pinline_comment)))(input)
    //many0(alt((entry, map(pinline_comment, )))(input)
}

pub fn parse_file(mut input: String) -> Result<Vec<Entry>, String> {
    input.retain(|c| !c.is_whitespace());
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
        char('('),
        separated_list0(char(','), groundterm), // TODO: whitespace
        char(')'),
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
fn apply2(input: &str) -> IResult<&str, Term> {
    let (input, head) = terminated(alphanumeric1, multispace0)(input)?;
    let (input, body) = separated_list0(multispace1, term2)(input)?;
    Ok((input, Apply(head.to_string(), body)))
}

fn term2(input: &str) -> IResult<&str, Term> {
    ws(alt((delimited(tag("("), term2, tag(")")), var, apply2)))(input)
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

fn forall2(input: &str) -> IResult<&str, Formula> {
    let (input, v) = delimited(
        ws(tag("forall")),
        alphanumeric0 , 
        ws(tag(",")))(input)?;
    let (input, f) = formula2(input)?;
    Ok((input, Formula::ForAll(v.to_string(), Box::new(f))))
}

fn exists2(input: &str) -> IResult<&str, Formula> {
    let (input, v) = delimited(
        ws(tag("exists")),
        alphanumeric0 , 
        ws(tag(",")))(input)?;
    let (input, f) = formula2(input)?;
    Ok((input, Formula::ForAll(v.to_string(), Box::new(f))))
}

fn conj2(input: &str) -> IResult<&str, Formula> {
    map(separated_list1(ws(alt((tag(","), tag("/\\"), tag("&")))), formula2simp), |fs| {
        Formula::Conj(fs)
    })(input)
}

fn atom2(input: &str) -> IResult<&str, Formula> {
    map(eqterm2, |eqt| Formula::Atom(eqt))(input)
}

fn implies2(input: &str) -> IResult<&str, Formula> {
    let (input, (h,c)) = tuple( (formula2 , preceded( ws(tag("=>")), formula2)) )(input)?;
    Ok((input, Formula::Implies(Box::new(h),Box::new(c))))
}

fn formula2(input: &str) -> IResult<&str, Formula> {
    ws(alt((
        delimited(tag("("), formula2, tag(")")),
        forall2, conj2)),
    )(input)
}
fn formula2simp(input: &str) -> IResult<&str, Formula> {
    ws(alt((
        delimited(tag("("), formula2, tag(")")),
        forall2, atom2)),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use Formula::*;
    #[test]
    fn parser2_test(){

        let f = "f".to_string();
        let x = Apply("x".to_string(), vec![]);

        assert_eq!(term2("(f x)").unwrap().1,  Apply(f.clone(), vec![x.clone()]) );

        assert_eq!(term2(" f x  ").unwrap().1,  Apply(f.clone(), vec![x.clone()]) );
        assert_eq!(eqterm2(" f x  ").unwrap().1,  Bare(Apply(f.clone(), vec![x.clone()]) ));
        assert_eq!(atom2(" f x  ").unwrap().1,  Atom(Bare(Apply(f.clone(), vec![x.clone()]) )));
        assert!(forall2(" f x  ").is_err());
        //assert!(conj2(" f x  ").is_err());
        
        assert_eq!(formula2(" f x  ").unwrap().1,  Conj(vec![Atom(Bare(Apply(f.clone(), vec![x.clone()]) ))]));

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
