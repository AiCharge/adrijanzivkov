<script setup lang="ts">
import { useVModel } from '@vueuse/core';
import type { HTMLAttributes } from 'vue';
import { computed, getCurrentInstance } from 'vue';
import { cn } from '@/lib/utils';

/**
 * A text field. With v-model it stays in sync with Vue. Without it, it is a
 * plain input: a password manager that sets .value (often without the events
 * Vue listens for) is not overwritten on the next render.
 */
const props = defineProps<{
    defaultValue?: string | number;
    modelValue?: string | number;
    class?: HTMLAttributes['class'];
}>();

const emits = defineEmits<{
    (e: 'update:modelValue', payload: string | number): void;
}>();

const passed = getCurrentInstance()?.vnode.props ?? {};
const isControlled =
    'modelValue' in passed ||
    'model-value' in passed ||
    'onUpdate:modelValue' in passed;

const modelValue = useVModel(props, 'modelValue', emits, {
    passive: true,
    defaultValue: props.defaultValue,
});

const nativeBindings = computed(() => {
    if (isControlled) {
        return { value: modelValue.value };
    }

    if (props.defaultValue === undefined) {
        return {};
    }

    return { defaultValue: props.defaultValue };
});

function onInput(event: Event): void {
    if (!isControlled) {
        return;
    }

    emits('update:modelValue', (event.target as HTMLInputElement).value);
}
</script>

<template>
    <input
        v-bind="nativeBindings"
        data-slot="input"
        :class="
            cn(
                'h-9 w-full min-w-0 rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none selection:bg-primary selection:text-primary-foreground file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm dark:bg-input/30',
                'focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50',
                'aria-invalid:border-destructive aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40',
                props.class,
            )
        "
        @input="onInput"
        @change="onInput"
    />
</template>
