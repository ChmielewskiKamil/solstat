use solang_parser;

fn main() {
    let file_contents = r#"
    
contract Contract0 {
    function addressInternalBalance(){
        uint256 bal = address(this).balance;
        bal++;
    }

    function addressExternalBalance(address addr) public {
        uint256 bal = address(addr).balance;
        bal++;
    }
}

    "#;

    let source_unit = solang_parser::parse(file_contents, 0).unwrap().0;

    println!("{:#?}", source_unit);
}
