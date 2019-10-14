//! # pest_ascii_tree
//!
//! This is a small helper crate useful for quickly debugging your pest
//! grammar.
//! The rules found by parsing the file are formated into an
//! [`ascii_tree`].
//!
//! It is useful, you you want to quickly debug your grammar without
//! having to write specialized code for handling the `Pairs` iterator
//! returned by your pest parser.
//!
//! Example, for whan an output might look like.
//! <pre>
//!  expr
//!  ├─ expr
//!  │  ├─ val "u"
//!  │  ├─ op "+"
//!  │  └─ expr
//!  │     ├─ val "v"
//!  │     ├─ op "+"
//!  │     └─ val "w"
//!  ├─ op "+"
//!  ├─ expr
//!  │  ├─ val "x"
//!  │  ├─ op "+"
//!  │  └─ val "y"
//!  ├─ op "+"
//!  └─ val "z"
//! </pre>
//!
//! Please, that the `EOI` rule is skipped.
//!
//! [`ascii_tree`]: ../ascii_tree/index.html

extern crate ascii_tree;
extern crate escape_string;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{error::Error, iterators::Pairs};

fn into_ascii_tree_nodes<R>(mut pairs: Pairs<R>) -> Vec<ascii_tree::Tree>
where
    R: pest::RuleType,
{
    let mut vec = Vec::new();

    while let Some(pair) = pairs.next() {
        let pair_content = pair.as_span().as_str().trim();
        let pair_rule = pair.as_rule();
        let inner_pairs = into_ascii_tree_nodes(pair.into_inner());

        let rule_name = format!("{:?}", pair_rule);
        if rule_name == "EOI" {
            continue;
        }

        let node;
        if inner_pairs.is_empty() {
            let leaf = format!(
                "{:?} \"{}\"",
                pair_rule,
                escape_string::escape(pair_content)
            );
            node = ascii_tree::Tree::Leaf(vec![leaf]);
        } else {
            node = ascii_tree::Tree::Node(rule_name, inner_pairs);
        }

        vec.push(node);
    }

    vec
}

/// Formats the parsing result by pest into an ascii_tree
/// into a [`String`].
///
/// # Error
/// If the internal call to [`ascii_tree::write_tree`] failed, the error
/// variant is passed to the caller.
///
/// # Examples
/// ```ignore
/// let result = pest_ascii_tree::into_ascii_tree(
///                  ExpressionParser::parse(Rule::expr, "(u + (v + w)) + (x + y) + z")?)?;
///     assert_eq!(
///         result,
///         String::new()
///             + " expr\n"
///             + " ├─ expr\n"
///             + " │  ├─ val \"u\"\n"
///             + " │  ├─ op \"+\"\n"
///             + " │  └─ expr\n"
///             + " │     ├─ val \"v\"\n"
///             + " │     ├─ op \"+\"\n"
///             + " │     └─ val \"w\"\n"
///             + " ├─ op \"+\"\n"
///             + " ├─ expr\n"
///             + " │  ├─ val \"x\"\n"
///             + " │  ├─ op \"+\"\n"
///             + " │  └─ val \"y\"\n"
///             + " ├─ op \"+\"\n"
///             + " └─ val \"z\"\n"
///     );
/// ```
///
/// [`String`]: https://doc.rust-lang.org/nightly/alloc/string/struct.String.html
/// [`ascii_tree::write_tree`]: ../ascii_tree/fn.write_tree.html
pub fn into_ascii_tree<R>(pairs: Pairs<R>) -> Result<String, std::fmt::Error>
where
    R: pest::RuleType,
{
    let nodes = into_ascii_tree_nodes(pairs);

    let mut output = String::new();

    match nodes.len() {
        0 => {}
        1 => {
            ascii_tree::write_tree(&mut output, nodes.first().unwrap())?;
        }
        _ => {
            let root = ascii_tree::Tree::Node(String::new(), nodes);
            ascii_tree::write_tree(&mut output, &root)?;

            if output.starts_with(" \n") {
                output = output.split_off(2);
            }
        }
    };

    Ok(output)
}

/// Prints the result returned by your pest Parser as an ascii tree.
///
/// # Errors
/// In case of an parsing error, the error is printed.
/// In case of a formating error, the error is printed.
///
/// # Examples
/// ```ignore
/// pest_ascii_tree::print_ascii_tree(
///                     ExpressionParser::parse(Rule::expr,
///                                             "(u + (v + w)) + (x + y) + z"));
/// ```
///
/// will result in the output
///
/// <pre>
///  expr
///  ├─ expr
///  │  ├─ val "u"
///  │  ├─ op "+"
///  │  └─ expr
///  │     ├─ val "v"
///  │     ├─ op "+"
///  │     └─ val "w"
///  ├─ op "+"
///  ├─ expr
///  │  ├─ val "x"
///  │  ├─ op "+"
///  │  └─ val "y"
///  ├─ op "+"
///  └─ val "z"
/// </pre>
///
pub fn print_ascii_tree<R>(parsing_result: Result<Pairs<R>, Error<R>>)
where
    R: pest::RuleType,
{
    match parsing_result {
        Ok(pairs) => match into_ascii_tree(pairs) {
            Ok(output) => {
                println!("{}", output);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::into_ascii_tree;
    use pest::Parser;

    #[derive(Parser)]
    #[grammar = "expression.pest"]
    struct ExpressionParser;

    #[test]
    fn it_works() {
        let result =
            into_ascii_tree(ExpressionParser::parse(Rule::expr, "a + b + c").unwrap()).unwrap();
        assert_eq!(
            result,
            String::new()
                + " expr\n"
                + " ├─ val \"a\"\n"
                + " ├─ op \"+\"\n"
                + " ├─ val \"b\"\n"
                + " ├─ op \"+\"\n"
                + " └─ val \"c\"\n"
        );

        let result =
            into_ascii_tree(ExpressionParser::parse(Rule::expr_root, "x + y + z").unwrap())
                .unwrap();
        assert_eq!(
            result,
            String::new()
                + " ├─ val \"x\"\n"
                + " ├─ op \"+\"\n"
                + " ├─ val \"y\"\n"
                + " ├─ op \"+\"\n"
                + " └─ val \"z\"\n"
        );

        let result = into_ascii_tree(ExpressionParser::parse(Rule::val, "m").unwrap()).unwrap();
        assert_eq!(result, String::new() + " val \"m\"\n");

        let result = into_ascii_tree(
            ExpressionParser::parse(Rule::expr, "(u + (v + w)) + (x + y) + z").unwrap(),
        )
        .unwrap();
        assert_eq!(
            result,
            String::new()
                + " expr\n"
                + " ├─ expr\n"
                + " │  ├─ val \"u\"\n"
                + " │  ├─ op \"+\"\n"
                + " │  └─ expr\n"
                + " │     ├─ val \"v\"\n"
                + " │     ├─ op \"+\"\n"
                + " │     └─ val \"w\"\n"
                + " ├─ op \"+\"\n"
                + " ├─ expr\n"
                + " │  ├─ val \"x\"\n"
                + " │  ├─ op \"+\"\n"
                + " │  └─ val \"y\"\n"
                + " ├─ op \"+\"\n"
                + " └─ val \"z\"\n"
        );

        let result = into_ascii_tree(
            ExpressionParser::parse(Rule::root, "(u + (v + w)) + (x + y) + z").unwrap(),
        )
        .unwrap();
        assert_eq!(
            result,
            String::new()
                + " expr\n"
                + " ├─ expr\n"
                + " │  ├─ val \"u\"\n"
                + " │  ├─ op \"+\"\n"
                + " │  └─ expr\n"
                + " │     ├─ val \"v\"\n"
                + " │     ├─ op \"+\"\n"
                + " │     └─ val \"w\"\n"
                + " ├─ op \"+\"\n"
                + " ├─ expr\n"
                + " │  ├─ val \"x\"\n"
                + " │  ├─ op \"+\"\n"
                + " │  └─ val \"y\"\n"
                + " ├─ op \"+\"\n"
                + " └─ val \"z\"\n"
        );

        let result = into_ascii_tree(
            ExpressionParser::parse(Rule::expr_root, "(u + (v + w)) + (x + y) + z").unwrap(),
        )
        .unwrap();
        assert_eq!(
            result,
            String::new()
                + " ├─ expr\n"
                + " │  ├─ val \"u\"\n"
                + " │  ├─ op \"+\"\n"
                + " │  └─ expr\n"
                + " │     ├─ val \"v\"\n"
                + " │     ├─ op \"+\"\n"
                + " │     └─ val \"w\"\n"
                + " ├─ op \"+\"\n"
                + " ├─ expr\n"
                + " │  ├─ val \"x\"\n"
                + " │  ├─ op \"+\"\n"
                + " │  └─ val \"y\"\n"
                + " ├─ op \"+\"\n"
                + " └─ val \"z\"\n"
        );
    }
}
