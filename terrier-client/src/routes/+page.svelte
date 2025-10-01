<script lang="ts">
    import ScottyLabsFilled from "@/components/icons/ScottyLabs_filled.svelte";
    import { client } from "@/lib/api";
    import { LogOut01Icon } from "@untitled-theme/icons-svelte";
    import { onMount } from "svelte";

    let hackathons = $state<any[]>([]);
    let isLoading = $state(true);

    onMount(async () => {
        const { data, response } = await client.GET("/hackathons");

        if (data && response.ok) {
            hackathons = data;
        }

        isLoading = false;
    });

    const apiUrl = import.meta.env.VITE_API_URL || "http://localhost:8080/api";
    const logout = () => {
        window.location.href = `${apiUrl}/auth/logout`;
    };
</script>

<div class="min-h-screen bg-secondary text-selected flex">
    <aside
        class="w-64 h-[calc(100vh-3.5rem)] mt-7 ml-7 p-4 rounded-4xl shadow-lg bg-primary"
    >
        <a href="/" class="mt-6 justify-center gap-2 flex">
            <ScottyLabsFilled class="my-auto" />
            <span class="text-2xl font-medium">Terrier</span>
        </a>

        <nav class="flex mt-8 flex-col gap-1">
            <button
                type="submit"
                onclick={logout}
                class="flex cursor-pointer gap-2.5 px-3 py-2 rounded-4xl text-selected"
            >
                <LogOut01Icon class="my-auto size-5" />
                <span class="font-medium">Logout</span>
            </button>
        </nav>
    </aside>

    <main class="flex-1 p-7 overflow-y-auto">
        <div class="w-full flex flex-col items-center gap-5 mt-10">
            <h1 class="text-2xl">Select a Hackathon</h1>

            {#if isLoading}
                <p class="text-center">Loading hackathons...</p>
            {:else if hackathons.length === 0}
                <div class="text-center">
                    <p class="text-gray-600 mb-4">No hackathons available</p>
                </div>
            {:else}
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {#each hackathons as hackathon}
                        <a
                            href="/h/{hackathon.slug}/dashboard"
                            class="block p-6 bg-primary rounded-lg shadow hover:shadow-lg transition-shadow"
                        >
                            <h2 class="text-2xl font-bold mb-2">
                                {hackathon.name}
                            </h2>
                            <p class="text-gray-600 mb-4">
                                {hackathon.description || "No description"}
                            </p>
                        </a>
                    {/each}
                </div>
            {/if}
        </div>
    </main>
</div>
