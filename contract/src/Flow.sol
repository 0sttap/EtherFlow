// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.26;

import {Collector, Ownable} from "./abstract/Collector.sol";

contract Flow is Collector {

    constructor () Ownable(msg.sender) {}

    receive() external payable {}
}
