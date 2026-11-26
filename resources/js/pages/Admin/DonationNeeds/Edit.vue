<script setup lang="ts">
import { index, update } from '@/actions/App/Http/Controllers/Admin/DonationNeedController';
import { show } from '@/actions/App/Http/Controllers/DonationNeedController';
import PageHeader from '@/components/admin/PageHeader.vue';
import RecordBackLink from '@/components/admin/RecordBackLink.vue';
import DonationNeedArchiveButton from '@/components/pages/admin/DonationNeedArchiveButton.vue';
import DonationNeedFields, { type DonationNeedFormData } from '@/components/pages/admin/DonationNeedFields.vue';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { Card, CardContent } from '@/components/ui/card';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import { formatDateTime } from '@/lib/formatters';
import { Link, setLayoutProps, useForm } from '@inertiajs/vue3';
import { ExternalLink, Save } from '@lucide/vue';

const props = defineProps<{
    donationNeed: DonationNeedInterface;
}>();

setLayoutProps({ breadcrumb: props.donationNeed.title });

const form = useForm<DonationNeedFormData>({
    title: props.donationNeed.title,
    description: props.donationNeed.description,
    sticky_content: props.donationNeed.sticky_content ?? '',
    needed_amount: String(props.donationNeed.needed_amount),
    image: null,
});

function submit(): void {
    form.transform((data) => ({ ...data, _method: 'put' })).post(update(props.donationNeed.id).url, {
        preserveScroll: true,
        onSuccess: () => form.reset('image'),
    });
}
</script>

<template>
    <div class="max-w-4xl space-y-6">
        <RecordBackLink :href="index().url" label="Донации" />

        <PageHeader :title="donationNeed.title" :description="`Додадено на ${formatDateTime(donationNeed.created_at)}`">
            <template #actions>
                <Badge :variant="donationNeed.archived_at ? 'warning' : 'success'">{{ donationNeed.archived_at ? 'Архивирано' : 'Активно' }}</Badge>
                <Button v-if="!donationNeed.archived_at" variant="outline" as-child>
                    <a :href="show(donationNeed.id).url" target="_blank" rel="noopener">
                        <ExternalLink />
                        Види на сајтот
                    </a>
                </Button>
                <DonationNeedArchiveButton :donation-need="donationNeed" />
            </template>
        </PageHeader>

        <form class="space-y-6" @submit.prevent="submit">
            <Card>
                <CardContent>
                    <DonationNeedFields v-model:form="form" :donation-need="donationNeed" />
                </CardContent>
            </Card>

            <div class="flex justify-end gap-2">
                <Button variant="outline" as-child>
                    <Link :href="index().url">Назад</Link>
                </Button>
                <Button type="submit" :disabled="form.processing">
                    <Save />
                    Зачувај
                </Button>
            </div>
        </form>
    </div>
</template>
