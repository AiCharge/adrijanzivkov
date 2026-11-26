<script setup lang="ts">
import { Button } from '@/components/ui/button';
import type { PaginatedInterface } from '@/interfaces/paginated-interface';
import { Link } from '@inertiajs/vue3';
import { ChevronLeft, ChevronRight } from '@lucide/vue';
import { computed } from 'vue';

/**
 * The footer of a paginated list: which rows are shown, the previous/next pages and a link to each page near the
 * current one, as Laravel's paginator lists them.
 */
const props = defineProps<{
    paginator: PaginatedInterface<unknown>;
}>();

// The paginator's first and last links are its own previous/next, shown here as buttons with icons.
const pages = computed(() => props.paginator.links.slice(1, -1));
</script>

<template>
    <div v-if="paginator.total > 0" class="text-muted-foreground flex flex-wrap items-center justify-between gap-2 text-sm">
        <span>{{ paginator.from }}–{{ paginator.to }} од {{ paginator.total }}</span>

        <nav v-if="paginator.last_page > 1" class="flex flex-wrap items-center gap-1" aria-label="Страници">
            <Button v-if="paginator.prev_page_url" variant="outline" size="sm" as-child>
                <Link :href="paginator.prev_page_url" preserve-scroll>
                    <ChevronLeft />
                    Претходна
                </Link>
            </Button>

            <template v-for="(page, index) in pages" :key="index">
                <Button v-if="page.url" :variant="page.active ? 'default' : 'ghost'" size="icon-sm" class="tabular-nums" as-child>
                    <Link :href="page.url" :aria-current="page.active ? 'page' : undefined" preserve-scroll>{{ page.label }}</Link>
                </Button>
                <span v-else class="px-1">…</span>
            </template>

            <Button v-if="paginator.next_page_url" variant="outline" size="sm" as-child>
                <Link :href="paginator.next_page_url" preserve-scroll>
                    Следна
                    <ChevronRight />
                </Link>
            </Button>
        </nav>
    </div>
</template>
