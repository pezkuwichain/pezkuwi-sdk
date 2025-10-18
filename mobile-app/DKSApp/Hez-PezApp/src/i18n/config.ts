import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import enTranslations from './locales/en.json';
import kmrTranslations from './locales/kmr.json';
import ckbTranslations from './locales/ckb.json';
import trTranslations from './locales/tr.json';
import faTranslations from './locales/fa.json';
import arTranslations from './locales/ar.json';

export const languages = {
  en: { name: 'English', flag: '🇬🇧', dir: 'ltr' },
  kmr: { name: 'Kurmancî', flag: '☀️', dir: 'ltr' },
  ckb: { name: 'سۆرانی', flag: '☀️', dir: 'rtl' },
  tr: { name: 'Türkçe', flag: '🇹🇷', dir: 'ltr' },
  fa: { name: 'فارسی', flag: '🇮🇷', dir: 'rtl' },
  ar: { name: 'العربية', flag: '🇸🇦', dir: 'rtl' }
};

const resources = {
  en: { translation: enTranslations },
  kmr: { translation: kmrTranslations },
  ckb: { translation: ckbTranslations },
  tr: { translation: trTranslations },
  fa: { translation: faTranslations },
  ar: { translation: arTranslations }
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: 'kmr',
    debug: false,
    interpolation: {
      escapeValue: false
    },
    detection: {
      order: ['localStorage', 'navigator', 'htmlTag'],
      caches: ['localStorage']
    }
  });

export default i18n;