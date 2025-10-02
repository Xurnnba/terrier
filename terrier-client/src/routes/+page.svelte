<script lang="ts">
    import ScottyLabsFilled from "@/components/icons/ScottyLabs_filled.svelte";
    import { client } from "@/lib/api";
    import { getAuthContext } from "@/lib/auth.svelte";
    import { PlusIcon } from "@untitled-theme/icons-svelte";
    import { onMount } from "svelte";

    const auth = getAuthContext();

    let hackathons = $state<any[]>([]);
    let isLoading = $state(true);
    let showCreateDialog = $state(false);

    onMount(async () => {
        const { data, response } = await client.GET("/hackathons/public");

        if (data && response.ok) {
            hackathons = data;
        }

        isLoading = false;
    });

    const create = () => {
        showCreateDialog = true;
    };
</script>

<div class="min-h-screen bg-secondary text-selected flex flex-col">
    <div class="m-7 mr-auto">
        <a href="/" class="gap-2 flex">
            <ScottyLabsFilled class="my-auto" />
            <span class="text-2xl font-medium">Terrier</span>
        </a>
    </div>

    <main class="mx-7">
        {#if auth.user?.is_admin}
            <button
                onclick={create}
                class="bg-selected text-primary cursor-pointer font-semibold px-5 py-3.5 flex gap-2 rounded-4xl mb-6"
            >
                <PlusIcon class="text-primary" />
                <span>New Hackathon</span>
            </button>
        {/if}

        <div class="w-full flex flex-col gap-5">
            {#if isLoading}
                <p class="text-center">Loading hackathons...</p>
            {:else if hackathons.length === 0}
                <p class="text-center">No hackathons found.</p>
            {:else}
                <div
                    class="grid grid-cols-auto gap-4 [grid-template-columns:repeat(auto-fill,minmax(16rem,16rem))]"
                >
                    {#each hackathons as hackathon}
                        <a
                            href="/h/{hackathon.slug}/dashboard"
                            class="size-64 p-6 bg-primary rounded-lg shadow-sm hover:shadow-md duration-250 transition-shadow"
                        >
                            <h2 class="text-2xl font-bold">{hackathon.name}</h2>
                            <p class="text-gray-600">
                                {hackathon.description || "No description"}
                            </p>
                        </a>
                    {/each}
                </div>
            {/if}
        </div>
    </main>
</div>
