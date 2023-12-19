use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "dts.pest"]
struct DTSParser;

use std::fs;


fn main() {
    let file = fs::read_to_string("zephyr.dts").expect("cannot read dts");

    let node_name_tests = [
        "simple-name",
        "zephyr,name",
        "some_name",
    ];
    for node_name_test in node_name_tests {
        DTSParser::parse(Rule::node_name, node_name_test).expect("successful node name parsing");
    }


    let prop_name_tests = [
        "simple-name",
        "zephyr,name",
        "some_name",
        "#address-cell",
    ];
    for prop_name_test in prop_name_tests {
        DTSParser::parse(Rule::property_name, prop_name_test).expect("successful property name parsing");
    }

    let cell_tests = [
        "0x1",
        "100",
    ];
    for cell_test in cell_tests {
        DTSParser::parse(Rule::cell, cell_test).expect("successful cell parsing");
    }

    let cell_array_tests = [
        "<100>",
        "<100 200>",
        "<0x1>",
        "<0x1 0x2>",
        "<0x1 100>",
    ];
    for cell_array_test in cell_array_tests {
        DTSParser::parse(Rule::cell_array, cell_array_test).expect("successful cell array parsing");
    }

    
    let prop_tests = [
        "#address-size = < 0x1 >;",
        "zephyr,console = &someconsole;",
    ];
    for prop_test in prop_tests {
        _ = DTSParser::parse(Rule::property, prop_test).expect("successful property parsing");
    }

    let node_tests = [
        "label: name@100 { };",
        "name@1fE { };",
        "name { };",
        "name { someprop = &someother; };",
        "name { someprop = &someother; child { someprop = &other; }; };",
        "name { some,prop = &someother; child { someprop = &other; }; };",

    ];
    for node_test in node_tests {
        _ = DTSParser::parse(Rule::node, node_test).expect("successful node parsing");
    }
    
    let dts = DTSParser::parse(Rule::dts, &file)
        .expect("successful dts parsing")
        .next().unwrap();

    println!("Parsed dts!");
}
