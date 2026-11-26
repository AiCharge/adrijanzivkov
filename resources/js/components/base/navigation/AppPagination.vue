<script setup lang="ts">
import { Link } from '@inertiajs/vue3';

interface PaginationLink {
    url: string | null;
    label: string;
    active: boolean;
}

interface Props {
    links: PaginationLink[];
}

defineProps<Props>();
</script>

<template>
    <nav class="mt-14 flex items-center justify-center">
        <div class="flex flex-wrap items-center justify-center gap-1.5">
            <template v-for="link in links" :key="link.label">
                <Link
                    v-if="link.url"
                    :href="link.url"
                    :class="
                        link.active
                            ? 'border-primary bg-primary text-white'
                            : 'border-gray-200 bg-white text-gray-700 hover:border-primary hover:text-primary'
                    "
                    class="min-w-10 rounded-full border px-4 py-2 text-center text-sm font-semibold transition-colors duration-200"
                    ><span v-html="link.label"></span
                ></Link>
                <span
                    v-else
                    class="min-w-10 cursor-not-allowed rounded-full border border-gray-100 px-4 py-2 text-center text-sm font-semibold text-gray-300"
                    ><span v-html="link.label"></span
                ></span>
            </template>
        </div>
    </nav>
</template>
