//! # pest_ascii_tree
//!
//! This is a small helper crate useful for quickly debugging your pest
//! grammar.
//!
//! For generating the output, [ascii_tree][1] is used.
//!
//! It is useful, you you want to quickly debug your grammar without
//! having to write specialized code for handling the `Pairs` iterator
//! returned by your pest parser.
//!
//! [1]: https://crates.io/crates/ascii_tree


extern crate ascii_tree;
extern crate pest;
extern crate escape_string;
#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pairs, error::Error};

fn as_ascii_tree_nodes<R>(mut pairs: Pairs<R>) -> Vec<ascii_tree::Tree> where
    R: pest::RuleType {

    let mut vec = Vec::new();

    while let Some(pair) = pairs.next() {
        let pair_content = pair.as_span().as_str().trim();
        let pair_rule = pair.as_rule();
        let inner_pairs = as_ascii_tree_nodes(pair.into_inner());

        let node;
        if inner_pairs.is_empty() {
            let leaf = vec![format!("{:?} \"{}\"", pair_rule, escape_string::escape(pair_content))];
            node = ascii_tree::Tree::Leaf(leaf);
        } else {
            node = ascii_tree::Tree::Node(format!("{:?}", pair_rule), inner_pairs);
        }

        vec.push(node);
    }

    vec
}

/// Returns the generated ascii_tree.
///
/// # Error
/// Returns an error, if the internal call to `ascii_tree::write_tree` failed.
fn as_ascii_tree_impl<R>(pairs: Pairs<R>) -> Result<String, std::fmt::Error> where
    R: pest::RuleType {

    let nodes = as_ascii_tree_nodes(pairs);

    let mut output = String::new();

    match nodes.len() {
        0 => {},
        1 => {ascii_tree::write_tree(&mut output, nodes.first().unwrap())?;},
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

/// Returns the generated ascii_tree.
///
/// Thought as a utility function for your tests.
///
/// # Error
/// Returns the error as string if formating the ascii tree failed.
pub fn as_ascii_tree<R>(pairs: Pairs<R>) -> String where
    R: pest::RuleType {

    match as_ascii_tree_impl(pairs) {
        Ok(s) => s,
        Err(e) => format!("{}", e),
    }
}

/// Prints the result returned by your pest Parser.
///
/// Otherwise, an ascii tree is printed.
/// In case of an error, the error is printed.
///
/// This is a convenience function.
/// For writing unittests, I recomment using `as_ascii_tree` instead.
pub fn print_as_ascii_tree<R>(parsing_result : Result<Pairs<R>, Error<R>>) where
    R: pest::RuleType {

    match parsing_result {
        Ok(pairs) => {
            match as_ascii_tree_impl(pairs) {
                Ok(output) => {println!("{}", output);}
                Err(e) => {eprintln!("{}", e);}
            }
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}


#[cfg(test)]
mod tests {

    use pest::Parser;
    use super::as_ascii_tree;

    #[derive(Parser)]
    #[grammar = "expression.pest"]
    struct ExpressionParser;

    #[test]
    fn it_works() {
        let result = as_ascii_tree(ExpressionParser::parse(Rule::expr, "a + b + c").expect("Expected expression to parse"));
        assert_eq!(result,
                   String::new() +
                   " expr\n" +
                   " ├─ val \"a\"\n" +
                   " ├─ op \"+\"\n" +
                   " ├─ val \"b\"\n" +
                   " ├─ op \"+\"\n" +
                   " └─ val \"c\"\n");

        let result = as_ascii_tree(ExpressionParser::parse(Rule::expr_root, "x + y + z").expect("Expected expression to parse"));
        assert_eq!(result,
                   String::new() +
                   " ├─ val \"x\"\n" +
                   " ├─ op \"+\"\n" +
                   " ├─ val \"y\"\n" +
                   " ├─ op \"+\"\n" +
                   " └─ val \"z\"\n");

        let result = as_ascii_tree(ExpressionParser::parse(Rule::val, "m").expect("Expected expression to parse"));
        assert_eq!(result,
                   String::new() +
                   " val \"m\"\n");

        let result = as_ascii_tree(ExpressionParser::parse(Rule::expr, "(u + (v + w)) + (x + y) + z").expect("Expected expression to parse"));
        assert_eq!(result,
                   String::new() +
                   " expr\n" +
                   " ├─ expr\n" +
                   " │  ├─ val \"u\"\n" +
                   " │  ├─ op \"+\"\n" +
                   " │  └─ expr\n" +
                   " │     ├─ val \"v\"\n" +
                   " │     ├─ op \"+\"\n" +
                   " │     └─ val \"w\"\n" +
                   " ├─ op \"+\"\n" +
                   " ├─ expr\n" +
                   " │  ├─ val \"x\"\n" +
                   " │  ├─ op \"+\"\n" +
                   " │  └─ val \"y\"\n" +
                   " ├─ op \"+\"\n" +
                   " └─ val \"z\"\n");

        let result = as_ascii_tree(ExpressionParser::parse(Rule::expr_root, "(u + (v + w)) + (x + y) + z").expect("Expected expression to parse"));
        assert_eq!(result,
                   String::new() +
                   " ├─ expr\n" +
                   " │  ├─ val \"u\"\n" +
                   " │  ├─ op \"+\"\n" +
                   " │  └─ expr\n" +
                   " │     ├─ val \"v\"\n" +
                   " │     ├─ op \"+\"\n" +
                   " │     └─ val \"w\"\n" +
                   " ├─ op \"+\"\n" +
                   " ├─ expr\n" +
                   " │  ├─ val \"x\"\n" +
                   " │  ├─ op \"+\"\n" +
                   " │  └─ val \"y\"\n" +
                   " ├─ op \"+\"\n" +
                   " └─ val \"z\"\n");
    }
}
