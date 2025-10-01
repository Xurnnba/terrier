<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
    import { getAuthContext, getHackathonContext } from "@/lib/auth.svelte";
    import { getHomeRoute, ROLES } from "@/lib/permissions";

    const auth = getAuthContext();
    const hackathon = getHackathonContext();
    const hackathonId = page.params.hackathonId!;

    // Redirect to the appropriate home route for their role
    if (auth.user?.is_admin) {
        goto(getHomeRoute(ROLES.ADMIN, hackathonId));
    } else if (hackathon.hackathonRole) {
        goto(getHomeRoute(hackathon.hackathonRole, hackathonId));
    } else {
        // Fallback if role isn't loaded yet
        goto(`/h/${hackathonId}/dashboard`);
    }
</script>

<div
    class="min-h-screen bg-secondary text-selected flex items-center justify-center"
>
    <p>Redirecting...</p>
</div>
