<script setup lang="ts">
import { destroy as logout } from '@/actions/App/Http/Controllers/Admin/AuthenticatedSessionController';
import { edit as editProfile } from '@/actions/App/Http/Controllers/Admin/ProfileController';
import AdminNavigation from '@/components/admin/AdminNavigation.vue';
import LogoLight from '@/components/logos/LogoLight.vue';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Breadcrumb, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator } from '@/components/ui/breadcrumb';
import { Button } from '@/components/ui/button';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Separator } from '@/components/ui/separator';
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarHeader,
    SidebarInset,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarProvider,
    SidebarRail,
    SidebarTrigger,
} from '@/components/ui/sidebar';
import { Toaster } from '@/components/ui/sonner';
import { usePageLoading } from '@/composables/usePageLoading';
import { initials } from '@/lib/initials';
import { adminNavigation } from '@/navigation/admin';
import { Link, router, usePage } from '@inertiajs/vue3';
import { ChevronsUpDown, ExternalLink, LogOut, Monitor, Moon, Sun, UserRound } from '@lucide/vue';
import { useColorMode } from '@vueuse/core';
import { computed, onMounted, onUnmounted } from 'vue';
import { toast } from 'vue-sonner';
import 'vue-sonner/style.css';

/**
 * `breadcrumb` is the record a page shows, set with setLayoutProps({ breadcrumb }); it follows the menu item.
 */
defineProps<{
    breadcrumb?: string;
}>();

const page = usePage();
const colorMode = useColorMode();
const resolvedColorMode = colorMode.state;

function isCurrent(href: string): boolean {
    const path = page.url.split('?')[0];

    return path === href || path.startsWith(`${href}/`);
}

const currentGroup = computed(() => adminNavigation.find((group) => group.items.some((item) => isCurrent(item.href))));
const currentItem = computed(() => currentGroup.value?.items.find((item) => isCurrent(item.href)));
const isOnProfile = computed(() => isCurrent(editProfile().url));

const userInitials = computed(() => initials(page.props.auth.user?.name ?? ''));

const { isLeaving } = usePageLoading();

function signOut(): void {
    router.post(logout().url);
}

function showFlash(message: string | undefined): void {
    if (message) {
        toast.success(message);
    }
}

let removeSuccessListener: (() => void) | undefined;

onMounted(() => {
    showFlash(page.props.flash?.success);

    removeSuccessListener = router.on('success', (event) => {
        showFlash((event.detail.page.props.flash as { success?: string } | undefined)?.success);
    });
});

onUnmounted(() => removeSuccessListener?.());
</script>

<template>
    <SidebarProvider>
        <Sidebar collapsible="icon" class="border-r-0">
            <SidebarHeader>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton size="lg" as="div" class="hover:bg-transparent active:bg-transparent">
                            <img
                                src="/favicon.svg"
                                alt=""
                                class="ring-sidebar-border hidden aspect-square size-8 rounded-md ring-1 group-data-[collapsible=icon]:block"
                            />
                            <div class="flex flex-col gap-1.5 group-data-[collapsible=icon]:hidden">
                                <LogoLight class="h-7 w-auto" aria-label="Фондација Адријан Живков" />
                                <span class="text-sidebar-primary text-[10px] font-semibold tracking-[0.18em] uppercase">Администрација</span>
                            </div>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarHeader>

            <SidebarContent>
                <AdminNavigation />
            </SidebarContent>

            <SidebarFooter>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <DropdownMenu>
                            <DropdownMenuTrigger as-child>
                                <SidebarMenuButton
                                    size="lg"
                                    class="ring-sidebar-border hover:bg-sidebar-accent data-[state=open]:bg-sidebar-accent ring-1 group-data-[collapsible=icon]:ring-0"
                                >
                                    <Avatar class="size-8 rounded-lg">
                                        <AvatarFallback class="bg-sidebar-primary text-sidebar-primary-foreground rounded-md text-xs font-semibold">
                                            {{ userInitials }}
                                        </AvatarFallback>
                                    </Avatar>
                                    <div class="grid flex-1 text-left leading-tight">
                                        <span class="truncate font-medium">{{ page.props.auth.user?.name }}</span>
                                        <span class="text-sidebar-foreground/70 truncate text-xs">{{ page.props.auth.user?.email }}</span>
                                    </div>
                                    <ChevronsUpDown class="ml-auto size-4" />
                                </SidebarMenuButton>
                            </DropdownMenuTrigger>

                            <DropdownMenuContent
                                side="top"
                                align="end"
                                :side-offset="8"
                                class="w-(--reka-dropdown-menu-trigger-width) min-w-56 rounded-lg"
                            >
                                <DropdownMenuLabel class="text-muted-foreground text-xs font-normal">Профил</DropdownMenuLabel>
                                <DropdownMenuItem as-child>
                                    <Link :href="editProfile().url">
                                        <UserRound />
                                        Мој профил
                                    </Link>
                                </DropdownMenuItem>
                                <DropdownMenuItem as-child>
                                    <a href="/" target="_blank" rel="noopener">
                                        <ExternalLink />
                                        Кон сајтот
                                    </a>
                                </DropdownMenuItem>
                                <DropdownMenuSeparator />
                                <DropdownMenuItem variant="destructive" @select="signOut">
                                    <LogOut />
                                    Одјава
                                </DropdownMenuItem>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarFooter>

            <SidebarRail />
        </Sidebar>

        <SidebarInset class="min-w-0">
            <header class="bg-card sticky top-0 z-10 flex h-14 shrink-0 items-center gap-2 border-b px-4 md:px-6">
                <SidebarTrigger class="-ml-1" />
                <Separator orientation="vertical" class="mr-2 data-[orientation=vertical]:h-4" />

                <Breadcrumb v-if="currentItem || isOnProfile">
                    <BreadcrumbList>
                        <template v-if="currentItem">
                            <BreadcrumbItem class="hidden md:block">{{ currentGroup?.label }}</BreadcrumbItem>
                            <BreadcrumbSeparator class="hidden md:block" />
                            <template v-if="breadcrumb">
                                <BreadcrumbItem>
                                    <BreadcrumbLink as-child>
                                        <Link :href="currentItem.href">{{ currentItem.label }}</Link>
                                    </BreadcrumbLink>
                                </BreadcrumbItem>
                                <BreadcrumbSeparator />
                                <BreadcrumbItem>
                                    <BreadcrumbPage class="max-w-48 truncate sm:max-w-80">{{ breadcrumb }}</BreadcrumbPage>
                                </BreadcrumbItem>
                            </template>
                            <BreadcrumbItem v-else>
                                <BreadcrumbPage>{{ currentItem.label }}</BreadcrumbPage>
                            </BreadcrumbItem>
                        </template>
                        <BreadcrumbItem v-else>
                            <BreadcrumbPage>Мој профил</BreadcrumbPage>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>

                <DropdownMenu>
                    <DropdownMenuTrigger as-child>
                        <Button variant="ghost" size="icon" class="ml-auto size-8" aria-label="Изглед">
                            <Sun class="dark:hidden" />
                            <Moon class="hidden dark:block" />
                        </Button>
                    </DropdownMenuTrigger>

                    <DropdownMenuContent align="end">
                        <DropdownMenuItem @select="colorMode = 'light'">
                            <Sun />
                            Светла
                        </DropdownMenuItem>
                        <DropdownMenuItem @select="colorMode = 'dark'">
                            <Moon />
                            Темна
                        </DropdownMenuItem>
                        <DropdownMenuItem @select="colorMode = 'auto'">
                            <Monitor />
                            Системска
                        </DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </header>

            <main
                :aria-busy="isLeaving"
                class="mx-auto w-full max-w-screen-2xl flex-1 px-4 py-8 transition-opacity md:px-8 md:py-10"
                :class="{ 'pointer-events-none opacity-50': isLeaving }"
            >
                <slot />
            </main>
        </SidebarInset>

        <Toaster position="top-right" rich-colors :theme="resolvedColorMode" />
    </SidebarProvider>
</template>
