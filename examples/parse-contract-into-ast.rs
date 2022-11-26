use solang_parser;

fn main() {
    let file_contents = r#"
    pragma solidity 0.8.0;
    
    contract Contract0 {
        function expensiveRevertStrings() {
            require(a < b, "long revert string over 32 bytes");
        }

        function cheapRevertStrings() {
            require(a < b, "a");
        }

        function noRevertMessage() {
            require(a < b);
        }
    }
    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
