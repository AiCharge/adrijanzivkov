<script setup lang="ts">
import { store } from '@/actions/App/Http/Controllers/Admin/AuthenticatedSessionController';
import { Button } from '@/components/ui/button';
import { Field, FieldError, FieldLabel } from '@/components/ui/field';
import { Input } from '@/components/ui/input';
import AdminAuthLayout from '@/layouts/AdminAuthLayout.vue';
import { Head, useForm } from '@inertiajs/vue3';

const form = useForm({
    email: '',
    password: '',
    remember: false,
});

function submit(): void {
    form.post(store().url, {
        onFinish: () => form.reset('password'),
    });
}
</script>

<template>
    <AdminAuthLayout title="Најава" description="Добредојдовте назад.">
        <Head title="Најава" />

        <form class="space-y-4" @submit.prevent="submit">
            <Field :data-invalid="!!form.errors.email">
                <FieldLabel for="email">Е-пошта</FieldLabel>
                <Input id="email" v-model="form.email" type="email" autocomplete="username" required autofocus :aria-invalid="!!form.errors.email" />
                <FieldError :errors="[form.errors.email]" />
            </Field>
            <Field :data-invalid="!!form.errors.password">
                <FieldLabel for="password">Лозинка</FieldLabel>
                <Input
                    id="password"
                    v-model="form.password"
                    type="password"
                    autocomplete="current-password"
                    required
                    :aria-invalid="!!form.errors.password"
                />
                <FieldError :errors="[form.errors.password]" />
            </Field>
            <label class="flex items-center gap-3 text-sm">
                <input v-model="form.remember" type="checkbox" class="size-4 shrink-0 accent-primary" />
                Запомни ме
            </label>
            <Button type="submit" class="w-full" :disabled="form.processing">Најави се</Button>
        </form>
    </AdminAuthLayout>
</template>
