<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
    import "@/app.css";
    import { client } from "@/lib/api";
    import { setAuthContext } from "@/lib/auth.svelte";
    import { canAccessRoute } from "@/lib/permissions";
    import { onMount } from "svelte";

    import Calendar from "@/components/icons/calendar.svelte";
    import ClipboardCheck from "@/components/icons/clipboard-check.svelte";
    import Cube01 from "@/components/icons/cube-01.svelte";
    import File05 from "@/components/icons/file-05.svelte";
    import Home03 from "@/components/icons/home-03.svelte";
    import LogOut01 from "@/components/icons/log-out-01.svelte";
    import MessageTextSquare01 from "@/components/icons/message-text-square-01.svelte";
    import QrCode01 from "@/components/icons/qr-code-01.svelte";
    import Scales01 from "@/components/icons/scales-01.svelte";
    import ScottyLabsFilled from "@/components/icons/ScottyLabs_filled.svelte";
    import Tool01 from "@/components/icons/tool-01.svelte";
    import User02 from "@/components/icons/user-02.svelte";
    import Users01 from "@/components/icons/users-01.svelte";

    const { children } = $props();
    const auth = setAuthContext();

    const allNavItems = [
        {
            href: "/dashboard",
            label: "Dashboard",
            icon: Home03,
        },
        {
            href: "/configuration",
            label: "Configuration",
            icon: Tool01,
        },
        {
            href: "/participants",
            label: "Participants",
            icon: Users01,
        },
        {
            href: "/schedule",
            label: "Schedule",
            icon: Calendar,
        },
        {
            href: "/messages",
            label: "Messages",
            icon: MessageTextSquare01,
        },
        {
            href: "/judging",
            label: "Judging",
            icon: Scales01,
        },
        {
            href: "/results",
            label: "Results",
            icon: ClipboardCheck,
        },
        {
            href: "/submission",
            label: "Project Submission",
            icon: Cube01,
        },
        {
            href: "/check-in",
            label: "Event Check-In",
            icon: QrCode01,
        },
        {
            href: "/profile",
            label: "Profile",
            icon: User02,
        },
        {
            href: "/application",
            label: "Application",
            icon: File05,
        },
    ];

    const navItems = $derived(
        auth.user
            ? allNavItems.filter((item) =>
                  canAccessRoute(auth.user!.role, item.href),
              )
            : [],
    );

    const currentPath = $derived(page.url.pathname);
    const apiUrl = import.meta.env.VITE_API_URL || "http://localhost:8080/api";

    onMount(async () => {
        const { data, response } = await client.GET("/auth/status");

        if (data && response.ok) {
            auth.user = data;
            auth.isLoading = false;

            // Check if user can access current route
            if (!canAccessRoute(data.role, currentPath)) {
                const firstAccessible = allNavItems.find((item) =>
                    canAccessRoute(data.role, item.href),
                );
                goto(firstAccessible?.href || "/dashboard");
            }
        } else {
            const appUrl = window.location.origin;
            const encodedUri = encodeURIComponent(`${appUrl}${currentPath}`);

            window.location.href = `${apiUrl}/auth/login?redirect_uri=${encodedUri}`;
        }
    });

    const logout = () => {
        window.location.href = `${apiUrl}/auth/logout`;
    };
</script>

{#if auth.isLoading}
    <div
        class="min-h-screen bg-secondary text-selected flex items-center justify-center"
    >
        <p>Authenticating...</p>
    </div>
{:else if auth.user}
    <div class="min-h-screen bg-secondary text-selected flex">
        <aside
            class="w-64 h-[calc(100vh-3.5rem)] mt-7 ml-7 p-4 rounded-4xl shadow-lg bg-primary"
        >
            <a href="/" class="mt-6 justify-center gap-2 flex">
                <ScottyLabsFilled class="my-auto" />
                <span class="text-2xl font-medium">Terrier</span>
            </a>

            <nav class="flex mt-8 flex-col gap-1">
                {#each navItems as item}
                    <a
                        href={item.href}
                        class="flex gap-2.5 px-3 py-2 rounded-4xl {currentPath ===
                        item.href
                            ? 'bg-selected text-primary'
                            : 'text-selected'}"
                    >
                        <item.icon class="my-auto size-5" />
                        <span class="font-medium">{item.label}</span>
                    </a>
                {/each}

                <button
                    type="submit"
                    onclick={logout}
                    class="flex cursor-pointer gap-2.5 px-3 py-2 rounded-4xl text-selected"
                >
                    <LogOut01 class="my-auto size-5" />
                    <span class="font-medium">Logout</span>
                </button>
            </nav>
        </aside>

        <main class="flex-1 p-7 overflow-y-auto">
            {@render children()}
        </main>
    </div>
{/if}
