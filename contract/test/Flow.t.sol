// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.26;

import {Test, console} from "forge-std/Test.sol";
import {Flow} from "../src/Flow.sol";

contract FlowTest is Test {
    Flow public flow;

    address[] _receivers = [address(0x1), address(0x2), address(0x3)];
    uint256[] _amounts = [1e18, 2e18, 3e18];
    uint256[] _percentages = [10, 20, 70];

    uint256 _amountToDistribute = 6e18;

    function setUp() public {
        flow = new Flow();
    }

    function test_lengthMismatchRevert() public {
        _receivers.pop();

        vm.expectRevert(Flow.LengthMismatch.selector);
        flow.distibuteETH{value: _amountToDistribute}(_receivers, _amounts);

        vm.expectRevert(Flow.LengthMismatch.selector);
        flow.distributeETHWithPercentages{value: _amountToDistribute}(_receivers, _percentages);

        vm.expectRevert(Flow.LengthMismatch.selector);
        flow.distributeERC20(_amountToDistribute, _receivers, _amounts);
    }
}
