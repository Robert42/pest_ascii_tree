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

fn as_ascii_tree<R>(pairs: Pairs<R>) -> Result<String, std::fmt::Error> where
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

pub fn print_as_ascii_tree<R>(parsing_result : Result<Pairs<R>, Error<R>>) where
    R: pest::RuleType {

    match parsing_result {
        Ok(pairs) => {
            match as_ascii_tree(pairs) {
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
        let result = as_ascii_tree(ExpressionParser::parse(Rule::expr, "a + b + c").expect("Expected expression to parse")).expect(" Expected ascii tree to build");
        assert_eq!(result,
                   String::new() +
                   " expr\n" +
                   " ├─ val \"a\"\n" +
                   " ├─ op \"+\"\n" +
                   " ├─ val \"b\"\n" +
                   " ├─ op \"+\"\n" +
                   " └─ val \"c\"\n");

        let result = as_ascii_tree(ExpressionParser::parse(Rule::expr_root, "x + y + z").expect("Expected expression to parse")).expect(" Expected ascii tree to build");
        assert_eq!(result,
                   String::new() +
                   " ├─ val \"x\"\n" +
                   " ├─ op \"+\"\n" +
                   " ├─ val \"y\"\n" +
                   " ├─ op \"+\"\n" +
                   " └─ val \"z\"\n");

        let result = as_ascii_tree(ExpressionParser::parse(Rule::val, "w").expect("Expected expression to parse")).expect(" Expected ascii tree to build");
        assert_eq!(result,
                   String::new() +
                   " val \"w\"\n");
    }
}
