<script setup lang="ts">
import AdminImageInput from '@/components/admin/AdminImageInput.vue';
import AdminRichEditor from '@/components/admin/AdminRichEditor.vue';
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field';
import { Input } from '@/components/ui/input';
import { Textarea } from '@/components/ui/textarea';
import type { NewsInterface } from '@/interfaces/news-interface';
import type { InertiaForm } from '@inertiajs/vue3';

export interface NewsFormData {
    title: string;
    short_description: string;
    content: string;
    main_image: File | null;
}

/**
 * The fields of a news post, shared by its add and edit pages. Editing passes the post.
 */
defineProps<{
    news?: NewsInterface;
}>();

const form = defineModel<InertiaForm<NewsFormData>>('form', { required: true });
</script>

<template>
    <FieldGroup class="grid gap-6">
        <Field :data-invalid="!!form.errors.title">
            <FieldLabel for="title">Наслов</FieldLabel>
            <Input id="title" v-model="form.title" required maxlength="255" :aria-invalid="!!form.errors.title" />
            <FieldDescription v-if="news">Адресата на сајтот останува /news/{{ news.slug }} кога насловот се менува.</FieldDescription>
            <FieldError :errors="[form.errors.title]" />
        </Field>

        <Field :data-invalid="!!form.errors.short_description">
            <FieldLabel for="short_description">Краток опис</FieldLabel>
            <Textarea
                id="short_description"
                v-model="form.short_description"
                rows="3"
                required
                maxlength="500"
                :aria-invalid="!!form.errors.short_description"
            />
            <FieldDescription>Се прикажува во листата на новости. Најмногу 500 знаци.</FieldDescription>
            <FieldError :errors="[form.errors.short_description]" />
        </Field>

        <Field :data-invalid="!!form.errors.main_image">
            <FieldLabel for="main_image">Главна слика</FieldLabel>
            <AdminImageInput id="main_image" v-model="form.main_image" :current-url="news?.main_image_url" :invalid="!!form.errors.main_image" />
            <FieldError :errors="[form.errors.main_image]" />
        </Field>

        <Field :data-invalid="!!form.errors.content">
            <FieldLabel for="content">Содржина</FieldLabel>
            <AdminRichEditor id="content" v-model="form.content" :invalid="!!form.errors.content" />
            <FieldError :errors="[form.errors.content]" />
        </Field>
    </FieldGroup>
</template>
