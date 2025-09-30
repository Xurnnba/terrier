<script lang="ts">
    import { goto } from "$app/navigation";
    import { getAuthContext } from "@/lib/auth.svelte";
    import { HOME_ROUTES, type Role } from "@/lib/permissions";

    const auth = getAuthContext();
    goto(HOME_ROUTES[auth.user!.role as Role]);
</script>
