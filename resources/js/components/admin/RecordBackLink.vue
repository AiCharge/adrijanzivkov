<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { Link } from '@inertiajs/vue3';
import { ArrowLeft } from '@lucide/vue';

/**
 * The link at the top of a record page back to its list.
 */
defineProps<{
    href: string;
    label: string;
}>();
</script>

<template>
    <Button variant="ghost" size="sm" class="-ml-3" as-child>
        <Link :href="href">
            <ArrowLeft />
            {{ label }}
        </Link>
    </Button>
</template>
