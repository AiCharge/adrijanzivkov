<script setup lang="ts">
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { ImageIcon, ImageUp, X } from '@lucide/vue';
import { computed, onBeforeUnmount, ref, watch } from 'vue';

/**
 * An image picker with a preview: the picked file, or else the image the record already has.
 */
const props = defineProps<{
    id?: string;
    currentUrl?: string | null;
    invalid?: boolean;
}>();

const model = defineModel<File | null>({ required: true });

const input = ref<HTMLInputElement | null>(null);
const previewUrl = ref<string | null>(null);

watch(model, (file) => {
    if (previewUrl.value) {
        URL.revokeObjectURL(previewUrl.value);
    }

    previewUrl.value = file ? URL.createObjectURL(file) : null;

    if (!file && input.value) {
        input.value.value = '';
    }
});

onBeforeUnmount(() => {
    if (previewUrl.value) {
        URL.revokeObjectURL(previewUrl.value);
    }
});

const displayedUrl = computed(() => previewUrl.value ?? props.currentUrl ?? null);

function onChange(event: Event): void {
    model.value = (event.target as HTMLInputElement).files?.[0] ?? null;
}
</script>

<template>
    <div
        :class="
            cn(
                'border-input bg-muted/30 flex flex-col gap-4 rounded-md border border-dashed p-4 sm:flex-row sm:items-center',
                invalid && 'border-destructive',
            )
        "
    >
        <div class="bg-muted flex aspect-video w-full shrink-0 items-center justify-center overflow-hidden rounded-md border sm:w-48">
            <img v-if="displayedUrl" :src="displayedUrl" alt="" class="size-full object-cover" />
            <ImageIcon v-else class="text-muted-foreground/50 size-8" />
        </div>
        <div class="flex min-w-0 flex-col gap-2 text-sm">
            <div class="flex flex-wrap items-center gap-2">
                <Button type="button" variant="outline" size="sm" @click="input?.click()">
                    <ImageUp />
                    {{ displayedUrl ? 'Замени слика' : 'Избери слика' }}
                </Button>
                <Button v-if="model" type="button" variant="ghost" size="sm" @click="model = null">
                    <X />
                    Поништи
                </Button>
            </div>
            <p v-if="model" class="text-muted-foreground truncate">{{ model.name }}</p>
            <p class="text-muted-foreground text-xs">JPG, PNG, WEBP или GIF, до 10 MB.</p>
            <input :id="id" ref="input" type="file" accept="image/*" class="sr-only" @change="onChange" />
        </div>
    </div>
</template>
