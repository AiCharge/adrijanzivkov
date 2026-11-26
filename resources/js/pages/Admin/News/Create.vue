<script setup lang="ts">
import { index, store } from '@/actions/App/Http/Controllers/Admin/NewsController';
import PageHeader from '@/components/admin/PageHeader.vue';
import RecordBackLink from '@/components/admin/RecordBackLink.vue';
import NewsFields, { type NewsFormData } from '@/components/pages/admin/NewsFields.vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Link, setLayoutProps, useForm } from '@inertiajs/vue3';
import { Plus } from '@lucide/vue';

setLayoutProps({ breadcrumb: 'Нова новост' });

const form = useForm<NewsFormData>({
    title: '',
    short_description: '',
    content: '',
    main_image: null,
});

function submit(): void {
    form.post(store().url);
}
</script>

<template>
    <div class="max-w-4xl space-y-6">
        <RecordBackLink :href="index().url" label="Новости" />

        <PageHeader title="Нова новост" description="Новоста се појавува на сајтот веднаш штом ќе се зачува." />

        <form class="space-y-6" @submit.prevent="submit">
            <Card>
                <CardContent>
                    <NewsFields v-model:form="form" />
                </CardContent>
            </Card>

            <div class="flex justify-end gap-2">
                <Button variant="outline" as-child>
                    <Link :href="index().url">Назад</Link>
                </Button>
                <Button type="submit" :disabled="form.processing">
                    <Plus />
                    Додај новост
                </Button>
            </div>
        </form>
    </div>
</template>
