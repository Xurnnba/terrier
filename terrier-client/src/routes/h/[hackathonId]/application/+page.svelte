<script lang="ts">
    import { getAuthContext } from "@/lib/auth.svelte";

    const auth = getAuthContext();
</script>

<div
    class="min-w-1/4 max-w-128 mt-10 mx-auto flex flex-col gap-10 justify-evenly"
>
    <div class="relative w-full min-h-8 justify-center flex">
        <h1 class="text-2xl">Application</h1>
        <div
            class="absolute flex gap-1.5 items-center border rounded-md border-border h-8 top-0 right-0 px-2.5 py-1.5"
        >
            <div class="w-2.5 h-2.5 rounded-full bg-selected"></div>
            <p class="text-sm font-semibold">Saved</p>
        </div>
    </div>

    <div class="bg-primary shadow-lg rounded-4xl w-full p-7">Content</div>

    <div class="flex justify-end">
        <button
            class="bg-selected text-primary font-semibold px-5 py-3.5 rounded-4xl"
        >
            Submit
        </button>
    </div>
</div>
