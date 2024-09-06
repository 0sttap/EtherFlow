// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.26;

import {Test, console} from "forge-std/Test.sol";
import {MockERC20} from "forge-std/mocks/MockERC20.sol";
import {Flow} from "../src/Flow.sol";
import {IFlow} from "../src/interfaces/IFlow.sol";

import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

contract FlowTest is Test {
    Flow public flow;
    MockERC20 public token;

    address[] _receivers = [makeAddr("user1"), makeAddr("user2"), makeAddr("user3")];
    uint256[] _amounts = [1e18, 2e18, 3e18];
    uint256[] _percentages = [10, 20, 70];

    address collector = makeAddr("collector");
    uint256 _amountToDistribute = 6e18;

    function setUp() public {
        flow = new Flow();
        token = new MockERC20();

        deal(address(token), address(this), _amountToDistribute);

        token.transfer(address(flow), _amountToDistribute);
    }

    function test_lengthMismatchRevert() public {
        _receivers.pop();

        vm.expectRevert(IFlow.LengthMismatch.selector);
        flow.distributeETH{value: _amountToDistribute}(_receivers, _amounts, true);

        vm.expectRevert(IFlow.LengthMismatch.selector);
        flow.distributeETH{value: _amountToDistribute}(_receivers, _percentages, false);

        vm.expectRevert(IFlow.LengthMismatch.selector);
        flow.distributeERC20(address(token), _amountToDistribute, _receivers, _amounts, true);

        vm.expectRevert(IFlow.LengthMismatch.selector);
        flow.distributeERC20(address(token), _amountToDistribute, _receivers, _percentages, false);
    }

    function test_invalidInputSharesRevert() public {
        _amounts[0] = 0;
        _percentages[0] = 0;

        vm.expectRevert(IFlow.InvalidInputShares.selector);
        flow.distributeETH{value: _amountToDistribute}(_receivers, _amounts, true);

        vm.expectRevert(IFlow.InvalidInputShares.selector);
        flow.distributeETH{value: _amountToDistribute}(_receivers, _percentages, false);

        vm.expectRevert(IFlow.InvalidInputShares.selector);
        flow.distributeERC20(address(token), _amountToDistribute, _receivers, _amounts, true);

        vm.expectRevert(IFlow.InvalidInputShares.selector);
        flow.distributeERC20(address(token), _amountToDistribute, _receivers, _percentages, false);
    }

    function test_distributeAndWithdrawETH() public {
        flow.distributeETH{value: _amountToDistribute}(_receivers, _amounts, true);

        for (uint256 i; i < _receivers.length; i++) {
            assertEq(address(_receivers[i]).balance, _amounts[i]);

            vm.prank(_receivers[i]);
            payable(address(flow)).transfer(_amounts[i]);
        }

        flow.withdrawETH(collector, _amountToDistribute, true);

        assertEq(address(collector).balance, _amountToDistribute);

        for (uint256 i; i < _receivers.length; i++) {
            assertEq(address(_receivers[i]).balance, 0);
        }

    }

    function test_distributeAndWithdrawETHWithPercentages() public {
        flow.distributeETH{value: _amountToDistribute}(_receivers, _percentages, false);

        for (uint256 i; i < _receivers.length; i++) {
            uint256 expectBalance = _amountToDistribute * _percentages[i] / 100;

            assertEq(address(_receivers[i]).balance, expectBalance);

            vm.prank(_receivers[i]);
            payable(address(flow)).transfer(expectBalance);
        }

        uint256 collectorBalance = address(collector).balance;

        flow.withdrawETH(collector, 65, false);

        assertEq(address(collector).balance, collectorBalance + _amountToDistribute * 65 / 100);
        assertEq(address(flow).balance, _amountToDistribute * 35 / 100);

        flow.withdrawETH(collector, 100, false);

        assertEq(address(collector).balance, collectorBalance + _amountToDistribute);
        assertEq(address(flow).balance, 0);
    }

    function test_distributeAndWithdrawERC20() public {
        flow.distributeERC20(address(token), _amountToDistribute, _receivers, _amounts, true);

        for (uint256 i; i < _receivers.length; i++) {
            assertEq(token.balanceOf(address(_receivers[i])), _amounts[i]);

            vm.prank(_receivers[i]);
            token.transfer(address(flow), _amounts[i]);
        }

        flow.withdrawERC20(address(token), collector, _amountToDistribute, true);

        assertEq(token.balanceOf(address(collector)), _amountToDistribute);

        for (uint256 i; i < _receivers.length; i++) {
            assertEq(token.balanceOf(address(_receivers[i])), 0);
        }
    }

    function test_distributeERC20WithPercentages() public {
        flow.distributeERC20(address(token), _amountToDistribute, _receivers, _percentages, false);

        for (uint256 i; i < _receivers.length; i++) {
            uint256 expectBalance = _amountToDistribute * _percentages[i] / 100;

            assertEq(token.balanceOf(address(_receivers[i])), expectBalance);

            vm.prank(_receivers[i]);
            token.transfer(address(flow), expectBalance);
        }

        uint256 collectorBalance = token.balanceOf(address(collector));

        flow.withdrawERC20(address(token), collector, 53, false);

        assertEq(token.balanceOf(address(collector)), collectorBalance + _amountToDistribute * 53 / 100);
        assertEq(token.balanceOf(address(flow)), _amountToDistribute * 47 / 100);

        flow.withdrawERC20(address(token), collector, 100, false);

        assertEq(token.balanceOf(address(collector)), collectorBalance + _amountToDistribute);
        assertEq(token.balanceOf(address(flow)), 0);
    }
}
