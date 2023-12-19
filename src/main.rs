use std::fs;
use nom::{number::complete::{u32, hex_u32} branch::*, combinator::*, error::*, sequence::*, IResult, Parser};


// pub struct Label(&'a str)
// pub struct NodeName(&'a str);
// pub struct PropertyName(&'a str);
// pub struct Cell(u32);

fn integer(input: &str) -> IResult<&str, u32> {
    alt((hex_u32, u32))(input)
}

// fn hex_digit(input: &str) -> IResult<&str, char> {
//     one_of("0123456789ABCDEFabcdef")
//     (input)
// }

// fn hex_number(input: &str) -> IResut<&str, u32> {
//     recognize(pair(tag("0x"), ))
// }

fn parse_cell(input: &str) -> IResult<&str, u32> {
    integer(input)
}

#[test]
fn test_parse_cell() { 
    assert_eq!(parse_cell("0x1aF"), Ok(("", 0x1AF)));
    assert_eq!(parse_cell("100"), Ok(("", 100)));
    //assert_eq!(parse_cell("0x10001004"), Ok("", Cell(0x10001004)));
    //assert_eq!(parse_cell("0x100010048"), Err("", Cell(0x10001004)));
}

// fn parse_prop_name(input: &str) -> IResult<&str, PropertyName> {
    
// }

// #[test]
// fn test_parse_prop_name() {
//     assert_eq!(parse_prop_name("#address-size"), Ok("", PropertyName("#address-size")));
//     assert_eq!(parse_prop_name("#legit-prop,name_of_prop?"), Ok("", PropertyName("#legit-prop,name_of_prop?")));
// }

// fn parse_node_name(input: &str) -> IResult<&str, NodeName> {
    
// }

// #[test]
// fn test_parse_node_name() {
//     assert_eq!(parse_prop_name("#address-size"), Ok("", PropertyName("#address-size")));
//     assert_eq!(parse_prop_name("#legit-prop,name_of_prop?"), Ok("", PropertyName("#legit-prop,name_of_prop?")));
// }

fn main() {
    println!("Parsed dts!");
}
