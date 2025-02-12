<script lang="ts">
    import { createEventDispatcher, getContext, onMount } from "svelte";
    import TokenInput from "../TokenInput.svelte";
    import type { DexId, OpenChat } from "openchat-client";
    import { _ } from "svelte-i18n";
    import Markdown from "../Markdown.svelte";
    import { random128 } from "openchat-shared";
    import { Record } from "@dfinity/candid/lib/cjs/idl";
    import CryptoSelector from "../CryptoSelector.svelte";
    import Legend from "../../Legend.svelte";
    import SwapProgress from "./SwapProgress.svelte";
    import ModalContent from "../../ModalContent.svelte";
    import ButtonGroup from "../../ButtonGroup.svelte";
    import Button from "../../Button.svelte";
    import BalanceWithRefresh from "../BalanceWithRefresh.svelte";
    import { mobileWidth } from "../../../stores/screenDimensions";
    import ErrorMessage from "../../ErrorMessage.svelte";
    import { i18nKey, type ResourceKey } from "../../../i18n/i18n";
    import Translatable from "../../Translatable.svelte";

    export let ledgerIn: string;

    const client = getContext<OpenChat>("client");
    const dispatch = createEventDispatcher();

    type State = "quote" | "swap" | "finished";
    type Result = "success" | "rateChanged" | "insufficientFunds" | "error" | undefined;

    let error: string | undefined = undefined;
    let amountIn: bigint = BigInt(0);
    let busy = false;
    let valid = false;
    let state: State = "quote";
    let result: Result = undefined;
    let validAmount = false;
    let ledgerOut: string | undefined;
    let swaps = {} as Record<string, DexId[]>;
    let message: string | undefined = undefined;
    let bestQuote: [DexId, bigint] | undefined = undefined;
    let swapId: bigint | undefined;

    $: cryptoLookup = client.enhancedCryptoLookup;
    $: detailsIn = $cryptoLookup[ledgerIn];
    $: detailsOut = ledgerOut !== undefined ? $cryptoLookup[ledgerOut] : undefined;
    $: anySwapsAvailable = Object.keys(swaps).length > 0 && detailsOut !== undefined;
    $: swapping = state === "swap" && busy;
    $: amountInText = client.formatTokens(amountIn, detailsIn.decimals);
    $: minAmountOut =
        bestQuote !== undefined ? (bestQuote[1] * BigInt(98)) / BigInt(100) : BigInt(0);

    $: {
        valid =
            anySwapsAvailable && validAmount && (state === "swap" ? bestQuote !== undefined : true);
    }

    $: title =
        state === "quote"
            ? i18nKey("tokenSwap.swapToken", { tokenIn: detailsIn.symbol })
            : i18nKey("tokenSwap.swapTokenTo", {
                  tokenIn: detailsIn.symbol,
                  tokenOut: detailsOut!.symbol,
              });

    $: cryptoBalanceStore = client.cryptoBalance;
    $: balanceIn = $cryptoBalanceStore[ledgerIn];
    $: remainingBalance =
        amountIn > BigInt(0) ? balanceIn - amountIn - detailsIn.transferFee : balanceIn;

    $: primaryButtonText = getPrimaryButtonText(state, result);

    onMount(() => loadSwaps(ledgerIn));

    function getPrimaryButtonText(state: State, result: Result): ResourceKey {
        let label;

        if (state === "finished") {
            label = result === "insufficientFunds" ? "back" : "requote";
        } else {
            label = state;
        }

        return i18nKey(`tokenSwap.${label}`);
    }

    function quote() {
        if (!valid) return;

        busy = true;
        error = undefined;
        result = undefined;
        swapId = undefined;

        client
            .getTokenSwapQuotes(ledgerIn, ledgerOut!, amountIn)
            .then((response) => {
                if (response.length === 0) {
                    error = $_("tokenSwap.noQuotes", { values: { tokenIn: detailsIn.symbol } });
                } else {
                    bestQuote = response[0];

                    const [dexId, quote] = bestQuote!;
                    const amountOutText = client.formatTokens(quote, detailsOut!.decimals);
                    const rate = (Number(amountOutText) / Number(amountInText)).toPrecision(3);
                    const dex = dexName(dexId);
                    const swapText = $_("tokenSwap.swap");
                    const minAmountOut = BigInt(10) * detailsOut!.transferFee;
                    const minAmountOutText = client.formatTokens(
                        minAmountOut,
                        detailsOut!.decimals,
                    );

                    let values = {
                        amountIn: amountInText,
                        tokenIn: detailsIn.symbol,
                        rate,
                        amountOut: amountOutText,
                        tokenOut: detailsOut!.symbol,
                        dex,
                        swap: swapText,
                        minAmountOut: minAmountOutText,
                    };

                    if (quote > minAmountOut) {
                        state = "swap";
                        message = $_("tokenSwap.swapInfo", { values });
                    } else {
                        error = $_("tokenSwap.quoteTooLow", { values });
                    }
                }
            })
            .catch((err) => {
                client.logError(`Error getting swap quotes for token: ${detailsIn.symbol}`, err);
                error = $_("tokenSwap.quoteError", { values: { tokenIn: detailsIn.symbol } });
            })
            .finally(() => (busy = false));
    }

    function swap() {
        if (!valid) return;

        busy = true;
        message = undefined;
        error = undefined;
        result = undefined;

        swapId = random128();

        client.swapTokens(swapId, ledgerIn, ledgerOut!, amountIn, minAmountOut, bestQuote![0]);
    }

    function dexName(dex: DexId): string {
        switch (dex) {
            case "icpswap":
                return "ICPSwap";
        }
    }

    function loadSwaps(ledger: string) {
        client.getTokenSwaps(ledger).then((results) => {
            ledgerOut = undefined;
            swaps = results;
        });
    }

    function onLedgerInSelected(ev: CustomEvent<{ ledger: string; urlFormat: string }>): void {
        loadSwaps(ev.detail.ledger);
    }

    function onSwapFinished(
        ev: CustomEvent<"success" | "rateChanged" | "insufficientFunds" | "error">,
    ) {
        busy = false;
        state = "finished";
        result = ev.detail;

        client.refreshAccountBalance(ledgerIn);
        client.refreshAccountBalance(ledgerOut!);
    }

    function onPrimaryClick() {
        if (state === "finished" && result === "insufficientFunds") {
            amountIn = BigInt(0);
            state = "quote";
        } else if (state === "quote" || result === "rateChanged") {
            quote();
        } else if (state === "swap") {
            swap();
        }
    }

    function onBalanceRefreshed() {
        error = undefined;
    }

    function onBalanceRefreshError(ev: CustomEvent<string>) {
        error = $_(ev.detail);
    }
</script>

<ModalContent>
    <span class="header" slot="header">
        <div class="main-title"><Translatable resourceKey={title} /></div>
        {#if state === "quote"}
            <BalanceWithRefresh
                ledger={ledgerIn}
                value={remainingBalance}
                label={i18nKey("cryptoAccount.shortBalanceLabel")}
                bold
                on:refreshed={onBalanceRefreshed}
                on:error={onBalanceRefreshError} />
        {/if}
    </span>
    <form class="body" slot="body">
        {#if state === "quote"}
            {#await client.swappableTokens() then swappableTokens}
                <div class="swap">
                    <div class="select-from">
                        <Legend label={i18nKey("cryptoAccount.transactionHeaders.from")} />
                        <div class="inner">
                            <CryptoSelector
                                filter={(t) => t.balance > 0 && swappableTokens.has(t.ledger)}
                                bind:ledger={ledgerIn}
                                on:select={onLedgerInSelected} />
                        </div>
                    </div>
                    <div class="amount">
                        <TokenInput
                            ledger={ledgerIn}
                            minAmount={detailsIn.transferFee * BigInt(100)}
                            maxAmount={detailsIn.balance}
                            bind:valid={validAmount}
                            bind:amount={amountIn} />
                    </div>
                    <div class="select-to">
                        <Legend label={i18nKey("cryptoAccount.transactionHeaders.to")} />
                        <div class="inner">
                            <CryptoSelector
                                filter={(t) => Object.keys(swaps).includes(t.ledger)}
                                bind:ledger={ledgerOut} />
                        </div>
                    </div>
                </div>
            {/await}
        {/if}

        {#if (swapping || state === "finished") && swapId !== undefined && detailsOut !== undefined && bestQuote !== undefined}
            <div>
                <SwapProgress
                    {swapId}
                    tokenIn={detailsIn.symbol}
                    tokenOut={detailsOut.symbol}
                    amountIn={amountInText}
                    decimalsOut={detailsOut.decimals}
                    dex={dexName(bestQuote[0])}
                    on:finished={onSwapFinished} />
            </div>
        {/if}

        {#if message !== undefined}
            <Markdown inline={false} text={message} />
        {/if}

        {#if error !== undefined}
            <ErrorMessage>{error}</ErrorMessage>
        {/if}
    </form>
    <span slot="footer">
        <ButtonGroup>
            {#if !swapping}
                <Button secondary tiny={$mobileWidth} on:click={() => dispatch("close")}
                    ><Translatable resourceKey={i18nKey("close")} /></Button>
            {/if}
            {#if result !== "success" && result !== "error"}
                <Button
                    disabled={busy || !valid}
                    loading={busy}
                    tiny={$mobileWidth}
                    on:click={onPrimaryClick}
                    ><Translatable resourceKey={primaryButtonText} /></Button>
            {/if}
        </ButtonGroup>
    </span>
</ModalContent>

<style lang="scss">
    :global(.swap input.amount-val) {
        border-radius: 0 !important;
        border: var(--bw) solid var(--bd) !important;
        border-left: none !important;
        border-right: none !important;
        height: 47px;
    }

    .header {
        display: flex;
        align-items: flex-start;
        justify-content: space-between;
        gap: $sp2;

        .main-title {
            flex: auto;
        }
    }

    .body {
        display: flex;
        flex-direction: column;
        gap: $sp4;
    }

    .swap {
        display: flex;
        justify-content: space-between;
        align-items: center;

        .inner {
            @include font(book, normal, fs-100);
            padding: 0 $sp4;
            border: var(--bw) solid var(--bd);
            background-color: var(--modal-bg);
            display: flex;
            height: 47px;
            align-items: center;
        }

        .select-from .inner {
            border-radius: var(--rd) 0 0 var(--rd);
        }

        .select-to .inner {
            border-radius: 0 var(--rd) var(--rd) 0;
        }

        .amount {
            flex-grow: 1;
        }
    }
</style>
