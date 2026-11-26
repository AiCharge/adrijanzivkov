<script setup lang="ts">
import { Button } from '@/components/ui/button';
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog';
import { ref } from 'vue';

/**
 * A dialog that asks before an action runs. The trigger goes in the default slot; `confirm` runs the action.
 */
defineProps<{
    title: string;
    description: string;
    confirmLabel: string;
    processing?: boolean;
}>();

const emit = defineEmits<{
    confirm: [close: () => void];
}>();

const open = ref(false);
</script>

<template>
    <Dialog v-model:open="open">
        <DialogTrigger as-child>
            <slot />
        </DialogTrigger>

        <DialogContent>
            <DialogHeader>
                <DialogTitle>{{ title }}</DialogTitle>
                <DialogDescription>{{ description }}</DialogDescription>
            </DialogHeader>

            <DialogFooter>
                <DialogClose as-child>
                    <Button type="button" variant="outline">Откажи</Button>
                </DialogClose>
                <Button type="button" :disabled="processing" @click="emit('confirm', () => (open = false))">{{ confirmLabel }}</Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
