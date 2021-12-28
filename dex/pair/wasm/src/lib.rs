////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![no_std]

elrond_wasm_node::wasm_endpoints! {
    pair
    (
        init
        acceptInformation
        acceptLiquidity
        addClone
        addLiquidity
        addTrustedSwapPair
        getAmountIn
        getAmountOut
        getBurnedTokenAmount
        getBurnedTokenAmountList
        getClones
        getEquivalent
        getExternSwapGasLimit
        getFeeDestinations
        getFeeState
        getFirstTokenId
        getGeneratedTokenAmount
        getGeneratedTokenAmountList
        getInfoShareMinBlocks
        getLastInfoShareEpoch
        getLiquidity
        getLpTokenIdentifier
        getOwnInfo
        getReceivedInfo
        getReserve
        getReservesAndTotalSupply
        getRouterManagedAddress
        getRouterOwnerManagedAddress
        getSecondTokenId
        getSpecialFee
        getState
        getTokensForGivenPosition
        getTotalFeePercent
        getTotalSupply
        getTransferExecGasLimit
        getTrustedSwapPairs
        getVirtualLiquidity
        getVirtualReserve
        getVirtualReservesAndTotalSupply
        getWhitelistedManagedAddresses
        pause
        removeLiquidity
        removeLiquidityAndBuyBackAndBurnToken
        removeTrustedSwapPair
        removeWhitelist
        resume
        setFeeOn
        setFeePercents
        setInfoShareMinBlocks
        setLpTokenIdentifier
        setStateActiveNoSwaps
        set_extern_swap_gas_limit
        set_transfer_exec_gas_limit
        shareInformation
        swapNoFeeAndForward
        swapTokensFixedInput
        swapTokensFixedOutput
        takeActionOnInformationReceive
        whitelist
    )
}

elrond_wasm_node::wasm_empty_callback! {}
