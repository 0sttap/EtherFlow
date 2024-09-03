// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.26;

interface IERC20 {
    function transfer(address recipient, uint256 amount) external returns (bool);
}

contract Flow {

    uint256 public constant PERCENTAGE_SCALE_FACTOR = 100;

    error LengthMismatch();
    error InvalidInputShares();

    enum DistributionType {
        AMOUNTS,
        PERCENTAGES
    }

    function distibuteETH(address[] calldata _receivers, uint256[] calldata _amounts) external payable {
        _distributeETH(_receivers, _amounts, DistributionType.AMOUNTS);
    }

    function distributeETHWithPercentages(address[] calldata _receivers, uint256[] calldata _percentages) external payable {
        _distributeETH(_receivers, _percentages, DistributionType.PERCENTAGES);
    }

    function distributeERC20(
        IERC20 _token,
        uint256 _amountToDistribute,
        address[] calldata _receivers,
        uint256[] calldata _amounts
    ) external {
        _distributeERC20(_token, _amountToDistribute, _receivers, _amounts, DistributionType.AMOUNTS);
    }

    function distributeERC20WithPercentages(
        IERC20 _token,
        uint256 _amountToDistribute,
        address[] calldata _receivers,
        uint256[] calldata _percentages
    ) external {
        _distributeERC20(_token, _amountToDistribute, _receivers, _percentages, DistributionType.PERCENTAGES);
    }

    function _distributeETH(address[] calldata _receivers, uint256[] calldata _shares, DistributionType _type) internal {
        uint256 receiversLength = _receivers.length;
        
        _validateDistribute(msg.value, receiversLength, _shares, _type);

        for (uint256 i; i < receiversLength; i++) {
            uint256 amount = _type == DistributionType.AMOUNTS
                ? _shares[i]
                : (msg.value * _shares[i]) / PERCENTAGE_SCALE_FACTOR;

            payable(_receivers[i]).transfer(amount);
        }
    }
    
    function _distributeERC20(
        IERC20 _token,
        uint256 _amountToDistribute,
        address[] calldata _receivers,
        uint256[] calldata _shares,
        DistributionType _type
    ) internal {
        uint256 receiversLength = _receivers.length;

        _validateDistribute(_amountToDistribute, receiversLength, _shares, _type);

        for (uint256 i; i < receiversLength; i++) {
            uint256 amount = _type == DistributionType.AMOUNTS
                ? _shares[i]
                : (_amountToDistribute * _shares[i]) / PERCENTAGE_SCALE_FACTOR;

            _token.transfer(_receivers[i], amount);
        }
    }

    function _validateDistribute(
        uint256 _amountToDistribute,
        uint256 _receiversLength,
        uint256[] calldata _shares,
        DistributionType _type
    ) internal pure {
        uint256 sharesLength = _shares.length;

        if (_receiversLength != sharesLength) revert LengthMismatch();

        uint256 totalShares = 0;
        for (uint256 i; i < sharesLength; i++) {
            unchecked { totalShares += _shares[i]; }
        }

        bool correctShares = _type == DistributionType.AMOUNTS ? totalShares == _amountToDistribute : totalShares == 100;

        if (!correctShares) revert InvalidInputShares();
    }
}
