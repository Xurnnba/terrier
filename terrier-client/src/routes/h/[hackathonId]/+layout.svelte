<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
    import "@/app.css";
    import { client } from "@/lib/api";
    import {
        getAuthContext,
        login,
        logout,
        setHackathonContext,
    } from "@/lib/auth.svelte";
    import { canAccessRoute } from "@/lib/permissions";

    import ErrorPage from "@/components/error-page.svelte";
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
    const hackathonId = $derived(page.params.hackathonId)!;

    const auth = getAuthContext();
    const hackathon = setHackathonContext();
    const currentPath = $derived(page.url.pathname);

    // Update hackathonId in context when it changes
    $effect(() => {
        hackathon.hackathonId = hackathonId;
    });

    let errorState = $state<{ status: number; message: string } | null>(null);
    let roleChecked = $state(false);

    // React to auth changes
    $effect(() => {
        if (!auth.isLoading && !roleChecked) {
            checkHackathonAccess();
        }
    });

    async function checkHackathonAccess() {
        if (!auth.user) {
            return login(currentPath);
        }

        const { data: roleData, response: roleResponse } = await client.GET(
            "/hackathons/{slug}/role",
            { params: { path: { slug: hackathonId } } },
        );

        if (roleData && roleResponse.ok) {
            hackathon.hackathonRole = roleData.role;
            hackathon.hackathonId = hackathonId;

            // Check if user can access current route
            if (!canAccessRoute(hackathon.hackathonRole, currentPath)) {
                const firstAccessible = allNavItems.find((item) =>
                    canAccessRoute(hackathon.hackathonRole!, item.href),
                );

                if (firstAccessible) {
                    goto(firstAccessible.href);
                } else {
                    errorState = {
                        status: 403,
                        message: "You do not have access to this hackathon",
                    };
                }
            }
        } else {
            errorState = {
                status: roleResponse.status === 404 ? 404 : 403,
                message:
                    roleResponse.status === 404
                        ? "Hackathon not found"
                        : "You do not have access to this hackathon",
            };
        }

        roleChecked = true;
    }

    const allNavItems = $derived([
        {
            href: `/h/${hackathonId}/dashboard`,
            label: "Dashboard",
            icon: Home03Icon,
        },
        {
            href: `/h/${hackathonId}/configuration`,
            label: "Configuration",
            icon: Tool01Icon,
        },
        {
            href: `/h/${hackathonId}/participants`,
            label: "Participants",
            icon: Users01Icon,
        },
        {
            href: `/h/${hackathonId}/schedule`,
            label: "Schedule",
            icon: CalendarIcon,
        },
        {
            href: `/h/${hackathonId}/messages`,
            label: "Messages",
            icon: MessageTextSquare01Icon,
        },
        {
            href: `/h/${hackathonId}/judging`,
            label: "Judging",
            icon: Scales01Icon,
        },
        {
            href: `/h/${hackathonId}/results`,
            label: "Results",
            icon: ClipboardCheckIcon,
        },
        {
            href: `/h/${hackathonId}/submission`,
            label: "Project Submission",
            icon: Cube01Icon,
        },
        {
            href: `/h/${hackathonId}/check-in`,
            label: "Event Check-In",
            icon: QrCode01Icon,
        },
        {
            href: `/h/${hackathonId}/profile`,
            label: "Profile",
            icon: User02Icon,
        },
        {
            href: `/h/${hackathonId}/application`,
            label: "Application",
            icon: File05Icon,
        },
    ]);

    const navItems = $derived(
        auth.user
            ? allNavItems.filter((item) =>
                  canAccessRoute(hackathon.hackathonRole!, item.href),
              )
            : [],
    );
</script>

{#if errorState}
    <ErrorPage {...errorState} />
{:else if auth.isLoading}
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
