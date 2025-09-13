import { createI18n } from 'vue-i18n';

import en from './locales/en.json';
import mk from './locales/mk.json';

const messages = {
    en,
    mk,
};

const i18n = createI18n({
    locale: 'mk',
    fallbackLocale: 'en',
    messages,
});

export default i18n;
