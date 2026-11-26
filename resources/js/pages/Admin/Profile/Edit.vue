<script setup lang="ts">
import { update } from '@/actions/App/Http/Controllers/Admin/ProfileController';
import PageHeader from '@/components/admin/PageHeader.vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Field, FieldError, FieldGroup, FieldLabel } from '@/components/ui/field';
import { Input } from '@/components/ui/input';
import { useForm, usePage } from '@inertiajs/vue3';
import { Save } from '@lucide/vue';

const page = usePage();

const form = useForm({
    name: page.props.auth.user?.name ?? '',
    email: page.props.auth.user?.email ?? '',
    current_password: '',
    password: '',
    password_confirmation: '',
});

function submit(): void {
    form.put(update().url, {
        preserveScroll: true,
        onSuccess: () => form.reset('current_password', 'password', 'password_confirmation'),
    });
}
</script>

<template>
    <div class="max-w-3xl space-y-6">
        <PageHeader title="Мој профил" description="Вашите податоци и лозинката за најава." />

        <form class="space-y-6" @submit.prevent="submit">
            <Card>
                <CardHeader>
                    <CardTitle>Лични податоци</CardTitle>
                </CardHeader>
                <CardContent>
                    <FieldGroup class="grid gap-6 md:grid-cols-2">
                        <Field :data-invalid="!!form.errors.name">
                            <FieldLabel for="name">Име</FieldLabel>
                            <Input id="name" v-model="form.name" autocomplete="name" required :aria-invalid="!!form.errors.name" />
                            <FieldError :errors="[form.errors.name]" />
                        </Field>
                        <Field :data-invalid="!!form.errors.email">
                            <FieldLabel for="email">Е-пошта</FieldLabel>
                            <Input
                                id="email"
                                v-model="form.email"
                                type="email"
                                autocomplete="username"
                                required
                                :aria-invalid="!!form.errors.email"
                            />
                            <FieldError :errors="[form.errors.email]" />
                        </Field>
                    </FieldGroup>
                </CardContent>
            </Card>

            <Card>
                <CardHeader>
                    <CardTitle>Промена на лозинка</CardTitle>
                    <CardDescription>Оставете ги полињата празни ако не сакате да ја промените лозинката.</CardDescription>
                </CardHeader>
                <CardContent>
                    <FieldGroup class="grid gap-6 md:grid-cols-2">
                        <Field :data-invalid="!!form.errors.current_password" class="md:col-span-2">
                            <FieldLabel for="current_password">Тековна лозинка</FieldLabel>
                            <Input
                                id="current_password"
                                v-model="form.current_password"
                                type="password"
                                autocomplete="current-password"
                                class="md:max-w-[calc(50%-0.75rem)]"
                                :aria-invalid="!!form.errors.current_password"
                            />
                            <FieldError :errors="[form.errors.current_password]" />
                        </Field>
                        <Field :data-invalid="!!form.errors.password">
                            <FieldLabel for="password">Нова лозинка</FieldLabel>
                            <Input
                                id="password"
                                v-model="form.password"
                                type="password"
                                autocomplete="new-password"
                                :aria-invalid="!!form.errors.password"
                            />
                            <FieldError :errors="[form.errors.password]" />
                        </Field>
                        <Field>
                            <FieldLabel for="password_confirmation">Потврди нова лозинка</FieldLabel>
                            <Input id="password_confirmation" v-model="form.password_confirmation" type="password" autocomplete="new-password" />
                        </Field>
                    </FieldGroup>
                </CardContent>
            </Card>

            <div class="flex justify-end">
                <Button type="submit" :disabled="form.processing">
                    <Save />
                    Зачувај
                </Button>
            </div>
        </form>
    </div>
</template>
