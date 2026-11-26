<script setup lang="ts">
import { index, store } from '@/actions/App/Http/Controllers/Admin/DonationNeedController';
import PageHeader from '@/components/admin/PageHeader.vue';
import RecordBackLink from '@/components/admin/RecordBackLink.vue';
import DonationNeedFields, { type DonationNeedFormData } from '@/components/pages/admin/DonationNeedFields.vue';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import { Link, setLayoutProps, useForm } from '@inertiajs/vue3';
import { Plus } from '@lucide/vue';

setLayoutProps({ breadcrumb: 'Нова донациска потреба' });

const form = useForm<DonationNeedFormData>({
    title: '',
    description: '',
    sticky_content: '',
    needed_amount: '',
    image: null,
});

function submit(): void {
    form.post(store().url);
}
</script>

<template>
    <div class="max-w-4xl space-y-6">
        <RecordBackLink :href="index().url" label="Донации" />

        <PageHeader title="Нова донациска потреба" description="Потребата се појавува на сајтот веднаш штом ќе се зачува." />

        <form class="space-y-6" @submit.prevent="submit">
            <Card>
                <CardContent>
                    <DonationNeedFields v-model:form="form" />
                </CardContent>
            </Card>

            <div class="flex justify-end gap-2">
                <Button variant="outline" as-child>
                    <Link :href="index().url">Назад</Link>
                </Button>
                <Button type="submit" :disabled="form.processing">
                    <Plus />
                    Додај потреба
                </Button>
            </div>
        </form>
    </div>
</template>
