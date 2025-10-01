<script lang="ts">
    let { status, message }: { status: number; message: string } = $props();
</script>

<div class="min-h-screen flex items-center justify-center bg-secondary">
    <div class="text-center">
        <h1 class="text-6xl font-bold mb-4">{status}</h1>
        <p class="text-xl text-gray-600 mb-8">{message}</p>

        <a
            href="/"
            class="px-6 py-3 bg-selected text-primary rounded-lg font-medium hover:opacity-90"
        >
            Back to Hackathons
        </a>
    </div>
</div>
