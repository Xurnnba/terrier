<script lang="ts">
import { getAuthContext } from "@/lib/auth.svelte";

const auth = getAuthContext();
</script>

<div>
    <p class="font-bold">Hello {auth.user?.name}</p>
</div>
