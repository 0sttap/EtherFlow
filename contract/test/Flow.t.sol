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

        token.approve(address(flow), _amountToDistribute);
    }

    function test_lengthMismatchRevert() public {
        _receivers.pop();

        vm.expectRevert(IFlow.LengthMismatch.selector);
        flow.distibuteETH{value: _amountToDistribute}(_receivers, _amounts);

        vm.expectRevert(IFlow.LengthMismatch.selector);
        flow.distributeETHWithPercentages{value: _amountToDistribute}(_receivers, _percentages);

        vm.expectRevert(IFlow.LengthMismatch.selector);
        flow.distributeERC20(IERC20(address(token)), _amountToDistribute, _receivers, _amounts);

        vm.expectRevert(IFlow.LengthMismatch.selector);
        flow.distributeERC20WithPercentages(IERC20(address(token)), _amountToDistribute, _receivers, _percentages);
    }

    function test_invalidInputSharesRevert() public {
        _amounts[0] = 0;
        _percentages[0] = 0;

        vm.expectRevert(IFlow.InvalidInputShares.selector);
        flow.distibuteETH{value: _amountToDistribute}(_receivers, _amounts);

        vm.expectRevert(IFlow.InvalidInputShares.selector);
        flow.distributeETHWithPercentages{value: _amountToDistribute}(_receivers, _percentages);

        vm.expectRevert(IFlow.InvalidInputShares.selector);
        flow.distributeERC20(IERC20(address(token)), _amountToDistribute, _receivers, _amounts);

        vm.expectRevert(IFlow.InvalidInputShares.selector);
        flow.distributeERC20WithPercentages(IERC20(address(token)), _amountToDistribute, _receivers, _percentages);
    }

    function test_distributeAndWithdrawETH() public {
        flow.distibuteETH{value: _amountToDistribute}(_receivers, _amounts);

        for (uint256 i; i < _receivers.length; i++) {
            assertEq(address(_receivers[i]).balance, _amounts[i]);

            vm.prank(_receivers[i]);
            payable(address(flow)).transfer(_amounts[i]);
        }

        flow.withdrawETH(collector, _amountToDistribute);

        assertEq(address(collector).balance, _amountToDistribute);

        for (uint256 i; i < _receivers.length; i++) {
            assertEq(address(_receivers[i]).balance, 0);
        }

    }

    function test_distributeAndWithdrawETHWithPercentages() public {
        flow.distributeETHWithPercentages{value: _amountToDistribute}(_receivers, _percentages);

        for (uint256 i; i < _receivers.length; i++) {
            uint256 expectBalance = _amountToDistribute * _percentages[i] / 100;

            assertEq(address(_receivers[i]).balance, expectBalance);

            vm.prank(_receivers[i]);
            payable(address(flow)).transfer(expectBalance);
        }

        flow.withdrawETHWithPercentage(collector, 65);

        assertEq(address(collector).balance, _amountToDistribute * 65 / 100);
        assertEq(address(flow).balance, _amountToDistribute * 35 / 100);

        flow.withdrawETHWithPercentage(collector, 100);

        assertEq(address(collector).balance, _amountToDistribute);
        assertEq(address(flow).balance, 0);
    }

    function test_distributeAndWithdrawERC20() public {
        flow.distributeERC20(IERC20(address(token)), _amountToDistribute, _receivers, _amounts);

        for (uint256 i; i < _receivers.length; i++) {
            assertEq(token.balanceOf(address(_receivers[i])), _amounts[i]);

            vm.prank(_receivers[i]);
            token.transfer(address(flow), _amounts[i]);
        }

        flow.withdrawERC20(IERC20(address(token)), collector, _amountToDistribute);

        assertEq(token.balanceOf(address(collector)), _amountToDistribute);

        for (uint256 i; i < _receivers.length; i++) {
            assertEq(token.balanceOf(address(_receivers[i])), 0);
        }
    }

    function test_distributeERC20WithPercentages() public {
        flow.distributeERC20WithPercentages(IERC20(address(token)), _amountToDistribute, _receivers, _percentages);

        for (uint256 i; i < _receivers.length; i++) {
            uint256 expectBalance = _amountToDistribute * _percentages[i] / 100;

            assertEq(token.balanceOf(address(_receivers[i])), expectBalance);

            vm.prank(_receivers[i]);
            token.transfer(address(flow), expectBalance);
        }

        flow.withdrawERC20WithPercentage(IERC20(address(token)), collector, 53);

        assertEq(token.balanceOf(address(collector)), _amountToDistribute * 53 / 100);
        assertEq(token.balanceOf(address(flow)), _amountToDistribute * 47 / 100);

        flow.withdrawERC20WithPercentage(IERC20(address(token)), collector, 100);

        assertEq(token.balanceOf(address(collector)), _amountToDistribute);
        assertEq(token.balanceOf(address(flow)), 0);
    }
}
