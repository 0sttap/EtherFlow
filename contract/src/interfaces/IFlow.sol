// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.26;

interface IFlow {
    error LengthMismatch();
    error InvalidInputShares();
    error NotSuccessfulCall();

    enum DistributionType {
        AMOUNTS,
        PERCENTAGES
    }
}