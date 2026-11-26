<script setup lang="ts">
import AdminImageInput from '@/components/admin/AdminImageInput.vue';
import AdminRichEditor from '@/components/admin/AdminRichEditor.vue';
import { Field, FieldDescription, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field';
import { Input } from '@/components/ui/input';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import type { InertiaForm } from '@inertiajs/vue3';

export interface DonationNeedFormData {
    title: string;
    description: string;
    sticky_content: string;
    needed_amount: string;
    image: File | null;
}

/**
 * The fields of a donation need, shared by its add and edit pages. Editing passes the donation need.
 */
defineProps<{
    donationNeed?: DonationNeedInterface;
}>();

const form = defineModel<InertiaForm<DonationNeedFormData>>('form', { required: true });
</script>

<template>
    <FieldGroup class="grid gap-6 md:grid-cols-2">
        <Field :data-invalid="!!form.errors.title">
            <FieldLabel for="title">Наслов</FieldLabel>
            <Input id="title" v-model="form.title" required maxlength="255" :aria-invalid="!!form.errors.title" />
            <FieldError :errors="[form.errors.title]" />
        </Field>

        <Field :data-invalid="!!form.errors.needed_amount">
            <FieldLabel for="needed_amount">Потребен износ (МКД)</FieldLabel>
            <Input
                id="needed_amount"
                v-model="form.needed_amount"
                type="number"
                inputmode="numeric"
                min="1"
                step="1"
                required
                :aria-invalid="!!form.errors.needed_amount"
            />
            <FieldError :errors="[form.errors.needed_amount]" />
        </Field>

        <Field :data-invalid="!!form.errors.image" class="md:col-span-2">
            <FieldLabel for="image">Слика</FieldLabel>
            <AdminImageInput id="image" v-model="form.image" :current-url="donationNeed?.image_url" :invalid="!!form.errors.image" />
            <FieldError :errors="[form.errors.image]" />
        </Field>

        <Field :data-invalid="!!form.errors.description" class="md:col-span-2">
            <FieldLabel for="description">Опис</FieldLabel>
            <AdminRichEditor id="description" v-model="form.description" :invalid="!!form.errors.description" />
            <FieldError :errors="[form.errors.description]" />
        </Field>

        <Field :data-invalid="!!form.errors.sticky_content" class="md:col-span-2">
            <FieldLabel for="sticky_content">Леплива содржина (десна страна)</FieldLabel>
            <AdminRichEditor id="sticky_content" v-model="form.sticky_content" :invalid="!!form.errors.sticky_content" />
            <FieldDescription>Се прикажува во страничниот дел на страницата за донацијата, на пр. број на сметка.</FieldDescription>
            <FieldError :errors="[form.errors.sticky_content]" />
        </Field>
    </FieldGroup>
</template>
