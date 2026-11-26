<script setup lang="ts">
import { destroy as unarchive, store as archive } from '@/actions/App/Http/Controllers/Admin/DonationNeedArchiveController';
import ConfirmDialog from '@/components/admin/ConfirmDialog.vue';
import { Button } from '@/components/ui/button';
import type { DonationNeedInterface } from '@/interfaces/donation-need-interface';
import { router } from '@inertiajs/vue3';
import { Archive, ArchiveRestore } from '@lucide/vue';
import { ref } from 'vue';

/**
 * Archives a donation need, so the site no longer shows it, or brings an archived one back, after asking first.
 */
const props = withDefaults(
    defineProps<{
        donationNeed: DonationNeedInterface;
        size?: 'default' | 'sm';
    }>(),
    { size: 'default' },
);

const processing = ref(false);

function confirm(close: () => void): void {
    const options = {
        preserveScroll: true,
        onStart: () => (processing.value = true),
        onFinish: () => (processing.value = false),
        onSuccess: close,
    };

    if (props.donationNeed.archived_at === null) {
        router.post(archive(props.donationNeed.id).url, {}, options);

        return;
    }

    router.delete(unarchive(props.donationNeed.id).url, options);
}
</script>

<template>
    <ConfirmDialog
        v-if="donationNeed.archived_at === null"
        title="Архивирај донациска потреба"
        description="Дали сте сигурни дека сакате да ја архивирате оваа донациска потреба? Таа нема да биде видлива на сајтот."
        confirm-label="Архивирај"
        :processing="processing"
        @confirm="confirm"
    >
        <Button variant="outline" :size="size" @click.stop>
            <Archive />
            Архивирај
        </Button>
    </ConfirmDialog>
    <ConfirmDialog
        v-else
        title="Врати донациска потреба"
        description="Дали сте сигурни дека сакате да ја вратите оваа донациска потреба? Таа ќе биде повторно видлива на сајтот."
        confirm-label="Врати"
        :processing="processing"
        @confirm="confirm"
    >
        <Button variant="outline" :size="size" @click.stop>
            <ArchiveRestore />
            Врати
        </Button>
    </ConfirmDialog>
</template>
