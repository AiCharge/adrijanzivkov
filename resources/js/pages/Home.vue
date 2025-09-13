<script setup lang="ts">
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
</script>

<template>
    <Head title="Почетна" />
    <Toaster richColors />

    <div>
        {{ t('test') }}
    </div>
</template>
