<script lang="ts">
    import { fade } from "svelte/transition";
    import NoChatSelected from "./NoChatSelected.svelte";
    import RecommendedGroups from "./RecommendedGroups.svelte";
    import ExploreCommunities from "./communities/explore/Explore.svelte";
    import type CurrentChatMessages from "./CurrentChatMessages.svelte";
    import CurrentChat from "./CurrentChat.svelte";
    import type { MultiUserChat, OpenChat } from "openchat-client";
    import { pathParams } from "../../routes";
    import { getContext } from "svelte";
    import AcceptRulesWrapper from "./AcceptRulesWrapper.svelte";
    import { currentTheme } from "../../theme/themes";
    import { layoutStore } from "../../stores/layout";
    import Loading from "../Loading.svelte";

    const client = getContext<OpenChat>("client");

    export let joining: MultiUserChat | undefined;
    export let currentChatMessages: CurrentChatMessages | undefined;

    $: selectedChatStore = client.selectedChatStore;
    $: selectedChatId = client.selectedChatId;
    $: eventsStore = client.eventsStore;
    $: filteredProposalsStore = client.filteredProposalsStore;
    $: noChat = $pathParams.kind !== "global_chat_selected_route";
</script>

<section
    class:offset={$layoutStore.showNav && !$layoutStore.showLeft}
    class:halloween={$currentTheme.name === "halloween"}>
    {#if $pathParams.kind === "explore_groups_route"}
        <RecommendedGroups {joining} on:joinGroup on:leaveGroup on:upgrade />
    {:else if $pathParams.kind === "communities_route"}
        <ExploreCommunities on:upgrade on:createCommunity />
    {:else if $pathParams.kind === "admin_route"}
        {#await import("./admin/Admin.svelte")}
            <div class="loading">
                <Loading />
            </div>
        {:then { default: Admin }}
            <Admin />
        {/await}
    {:else if $selectedChatId === undefined}
        {#if noChat}
            <div class="no-chat" in:fade>
                <NoChatSelected on:newchat />
            </div>
        {/if}
    {:else if $selectedChatStore !== undefined}
        <AcceptRulesWrapper
            let:sendMessageWithAttachment
            let:forwardMessage
            let:retrySend
            let:sendMessageWithContent
            messageContext={{ chatId: $selectedChatStore.id }}>
            <CurrentChat
                bind:currentChatMessages
                {joining}
                chat={$selectedChatStore}
                events={$eventsStore}
                filteredProposals={$filteredProposalsStore}
                on:successfulImport
                on:clearSelection
                on:leaveGroup
                on:replyPrivatelyTo
                on:showInviteGroupUsers
                on:showProposalFilters
                on:makeProposal
                on:showGroupMembers
                on:chatWith
                on:joinGroup
                on:upgrade
                on:toggleMuteNotifications
                on:goToMessageIndex
                on:convertGroupToCommunity
                on:retrySend={retrySend}
                on:sendMessageWithContent={sendMessageWithContent}
                on:sendMessageWithAttachment={sendMessageWithAttachment}
                on:forwardMessage={forwardMessage}
                on:forward />
        </AcceptRulesWrapper>
    {/if}
</section>

<style lang="scss">
    .no-chat {
        height: 100%;
    }

    section {
        min-width: 400px;
        overflow: auto;
        overflow-x: hidden;
        flex: 13;
        background: none;
        padding: 0;

        @include mobile() {
            min-width: unset;
        }

        &.offset {
            margin-inline-start: toRem(80);
            @include mobile() {
                margin-inline-start: toRem(60);
            }
        }

        &.halloween::after {
            @include cobweb();
            bottom: 0;
            right: 0;
            transform: scaleY(-1);
        }
    }
</style>
