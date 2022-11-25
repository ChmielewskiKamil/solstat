pragma solidity 0.8.0;

contract Contract0 {
    function addressInternalBalance() {
        uint256 bal = address(this).balance;
        bal++;
    }

    function addressExternalBalance(address addr) public {
        uint256 bal = address(addr).balance;
        bal++;
    }
}
