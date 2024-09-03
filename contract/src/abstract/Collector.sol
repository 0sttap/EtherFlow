// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.26;

import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

import {Distribute, IERC20} from "./Distribute.sol";

abstract contract Collector is Distribute, Ownable {
    function withdrawETH(address _to, uint256 _amount) external onlyOwner {
        (bool success, ) = _to.call{value: _amount}("");
        if (!success) revert NotSuccessfulCall();
    }

    function withdrawETHWithPercentage(address _to, uint256 _percentage) external onlyOwner {
        uint256 amount = (address(this).balance * _percentage) / PERCENTAGE_SCALE_FACTOR;

        (bool success, ) = _to.call{value: amount}("");
        if (!success) revert NotSuccessfulCall();
    }

    function withdrawERC20(IERC20 _token, address _to, uint256 _amount) external onlyOwner {
        _token.transfer(_to, _amount);
    }

    function withdrawERC20WithPercentage(IERC20 _token, address _to, uint256 _percentage) external onlyOwner {
        uint256 amount = (_token.balanceOf(address(this)) * _percentage) / PERCENTAGE_SCALE_FACTOR;

        _token.transfer(_to, amount);
    }
}
