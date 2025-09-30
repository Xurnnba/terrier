<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
    import "@/app.css";
    import { client } from "@/lib/api";
    import { setAuthContext } from "@/lib/auth.svelte";
    import { canAccessRoute } from "@/lib/permissions";
    import { onMount } from "svelte";

    import ScottyLabsFilled from "@/components/icons/ScottyLabs_filled.svelte";
    import {
        CalendarIcon,
        ClipboardCheckIcon,
        Cube01Icon,
        File05Icon,
        Home03Icon,
        LogOut01Icon,
        MessageTextSquare01Icon,
        QrCode01Icon,
        Scales01Icon,
        Tool01Icon,
        User02Icon,
        Users01Icon,
    } from "@untitled-theme/icons-svelte";

    const { children } = $props();
    const auth = setAuthContext();

    const allNavItems = [
        {
            href: "/dashboard",
            label: "Dashboard",
            icon: Home03Icon,
        },
        {
            href: "/configuration",
            label: "Configuration",
            icon: Tool01Icon,
        },
        {
            href: "/participants",
            label: "Participants",
            icon: Users01Icon,
        },
        {
            href: "/schedule",
            label: "Schedule",
            icon: CalendarIcon,
        },
        {
            href: "/messages",
            label: "Messages",
            icon: MessageTextSquare01Icon,
        },
        {
            href: "/judging",
            label: "Judging",
            icon: Scales01Icon,
        },
        {
            href: "/results",
            label: "Results",
            icon: ClipboardCheckIcon,
        },
        {
            href: "/submission",
            label: "Project Submission",
            icon: Cube01Icon,
        },
        {
            href: "/check-in",
            label: "Event Check-In",
            icon: QrCode01Icon,
        },
        {
            href: "/profile",
            label: "Profile",
            icon: User02Icon,
        },
        {
            href: "/application",
            label: "Application",
            icon: File05Icon,
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
                    <LogOut01Icon class="my-auto size-5" />
                    <span class="font-medium">Logout</span>
                </button>
            </nav>
        </aside>

        <main class="flex-1 p-7 overflow-y-auto">
            {@render children()}
        </main>
    </div>
{/if}
