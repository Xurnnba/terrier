<script lang="ts">
    import { page } from "$app/state";
    import "@/app.css";
    import { client } from "@/lib/api";
    import { login, setAuthContext } from "@/lib/auth.svelte";
    import { onMount } from "svelte";

    const { children } = $props();

    const auth = setAuthContext();
    const currentPath = $derived(page.url.pathname);

    onMount(async () => {
        const { data, response } = await client.GET("/auth/status");

        if (data && response.ok) {
            auth.user = data;
        } else {
            return login(currentPath);
        }

        auth.isLoading = false;
    });
</script>

{@render children()}
