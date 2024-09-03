// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.26;

import {Script, console} from "forge-std/Script.sol";

import {Flow} from "../src/Flow.sol";
/**
    forge script deploy/Flow.s.sol --ffi --broadcast
 */
contract FlowDeploy is Script {
    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");

        vm.startBroadcast(deployerPrivateKey);
        Flow flow = new Flow();
        vm.stopBroadcast();

        console.log("Flow deployed at:", address(flow));
    }
}
