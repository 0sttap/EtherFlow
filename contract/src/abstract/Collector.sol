// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.26;

import {Ownable} from "openzeppelin-contracts/contracts/access/Ownable.sol";

import {Distribute, IERC20} from "./Distribute.sol";

abstract contract Collector is Distribute, Ownable {
    function withdrawETH(address _to, uint256 _shares, bool _fixed) external onlyOwner {
        uint256 amount = _fixed ? _shares : (address(this).balance * _shares) / PERCENTAGE_SCALE_FACTOR;

        (bool success, ) = _to.call{value: amount}("");
        if (!success) revert NotSuccessfulCall();
    }

    function withdrawERC20(address _token, address _to, uint256 _shares, bool _fixed) external onlyOwner {
        uint256 amount = _fixed
            ? _shares
            : (IERC20(_token).balanceOf(address(this)) * _shares) / PERCENTAGE_SCALE_FACTOR;

        IERC20(_token).transfer(_to, amount);
    }
}
