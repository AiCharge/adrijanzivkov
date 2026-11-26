<script setup lang="ts">
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '@/components/ui/collapsible';
import {
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    useSidebar,
} from '@/components/ui/sidebar';
import type { AdminNavigationGroupInterface } from '@/interfaces/admin-navigation-interface';
import { adminNavigation } from '@/navigation/admin';
import { Link, usePage } from '@inertiajs/vue3';
import { ChevronRight } from '@lucide/vue';
import { useStorage } from '@vueuse/core';
import { watch } from 'vue';

/**
 * The menu's groups, each folding open or closed. The group of the current page is always opened, every other one
 * keeps the admin's choice (remembered in this browser) or how it starts. With the sidebar folded to icons, every
 * screen shows. On a phone the sidebar closes when another page opens.
 */
const page = usePage();
const { state, setOpenMobile } = useSidebar();
const openGroups = useStorage<Record<string, boolean>>('admin-navigation-open', {});

function isCurrent(href: string): boolean {
    const path = page.url.split('?')[0];

    return path === href || path.startsWith(`${href}/`);
}

function holdsCurrentPage(group: AdminNavigationGroupInterface): boolean {
    return group.items.some((item) => isCurrent(item.href));
}

function isOpen(group: AdminNavigationGroupInterface): boolean {
    return state.value === 'collapsed' || (openGroups.value[group.key] ?? group.startsOpen);
}

function setOpen(group: AdminNavigationGroupInterface, open: boolean): void {
    openGroups.value = { ...openGroups.value, [group.key]: open };
}

watch(
    () => page.url,
    () => adminNavigation.filter((group) => holdsCurrentPage(group)).forEach((group) => setOpen(group, true)),
    { immediate: true },
);

// Not immediate: on a phone this menu is made when the sidebar opens, and would close it at once.
watch(
    () => page.url,
    () => setOpenMobile(false),
);
</script>

<template>
    <Collapsible
        v-for="group in adminNavigation"
        :key="group.key"
        :open="isOpen(group)"
        class="group/collapsible"
        @update:open="(open) => setOpen(group, open)"
    >
        <SidebarGroup>
            <SidebarGroupLabel
                as-child
                class="text-sidebar-foreground/55 hover:text-sidebar-accent-foreground text-[11px] font-semibold tracking-wider uppercase"
            >
                <CollapsibleTrigger>
                    {{ group.label }}
                    <ChevronRight class="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90" />
                </CollapsibleTrigger>
            </SidebarGroupLabel>

            <CollapsibleContent>
                <SidebarGroupContent>
                    <SidebarMenu>
                        <SidebarMenuItem v-for="item in group.items" :key="item.href">
                            <SidebarMenuButton
                                as-child
                                :is-active="isCurrent(item.href)"
                                :tooltip="item.label"
                                class="text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground data-[active=true]:bg-sidebar-accent data-[active=true]:text-sidebar-accent-foreground data-[active=true]:before:bg-sidebar-primary data-[active=true]:[&>svg]:text-sidebar-primary relative h-9 transition-colors data-[active=true]:font-medium data-[active=true]:before:absolute data-[active=true]:before:inset-y-1.5 data-[active=true]:before:left-0 data-[active=true]:before:w-0.5 data-[active=true]:before:rounded-full"
                            >
                                <Link :href="item.href">
                                    <component :is="item.icon" />
                                    <span>{{ item.label }}</span>
                                </Link>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </SidebarMenu>
                </SidebarGroupContent>
            </CollapsibleContent>
        </SidebarGroup>
    </Collapsible>
</template>
