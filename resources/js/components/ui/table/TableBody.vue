<script setup lang="ts">
import type { HTMLAttributes } from 'vue';
import { useTemplateRef } from 'vue';
import TableCell from './TableCell.vue';
import TableRow from './TableRow.vue';
import { Skeleton } from '@/components/ui/skeleton';
import { usePageLoading } from '@/composables/usePageLoading';
import { cn } from '@/lib/utils';

const props = defineProps<{
    class?: HTMLAttributes['class'];
}>();

const { isReloading } = usePageLoading();
const body = useTemplateRef('body');

// While the page fetches new rows, placeholder rows fill the table, one cell per header column.
const columnCount = (): number =>
    body.value?.closest('table')?.tHead?.rows[0]?.cells.length ?? 1;
</script>

<template>
    <tbody
        ref="body"
        data-slot="table-body"
        :aria-busy="isReloading"
        :class="cn('[&_tr:last-child]:border-0', props.class)"
    >
        <template v-if="isReloading">
            <TableRow v-for="row in 5" :key="row" class="hover:bg-transparent">
                <TableCell
                    v-for="column in columnCount()"
                    :key="column"
                    class="h-12 first:pl-4 last:pr-4"
                >
                    <Skeleton class="h-4 w-full min-w-8" />
                </TableCell>
            </TableRow>
        </template>
        <slot v-else />
    </tbody>
</template>
