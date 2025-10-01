<script lang="ts">
    import { page } from "$app/state";
    import ErrorPage from "@/components/error-page.svelte";
</script>

<ErrorPage
    status={page.status}
    message={page.error?.message || "An error occurred"}
/>
