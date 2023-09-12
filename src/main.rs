extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "ident.pest"]
struct IdentParser;

use pest::iterators::Pair;

fn dump_pair(pair: &Pair<'_, Rule>, indent: usize) {
        println!("{}Rule:    {:?}", " ".repeat(indent), pair.as_rule());
        // println!("{}Span:    {:?}", " ".repeat(indent), pair.as_span());
        let text = pair.as_str();
        if text.len() < 40 {
           println!("{}Text:    {}", " ".repeat(indent), pair.as_str());
        }
        for inner_pair in pair.clone().into_inner() {
            dump_pair(&inner_pair, indent+1);
        }
}


fn main() {
let ipv4_def = include_str!("ipv4.pla");
    let pairs = IdentParser::parse(Rule::file, ipv4_def).unwrap_or_else(|e| panic!("{}: {:#?}", &e, &e));
    // println!("{:#?}", &pairs);
    for pair in pairs.clone() {
        dump_pair(&pair, 0);
    }

            /*
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
            */
}
