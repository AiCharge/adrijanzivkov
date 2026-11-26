import dayjs from '@/lib/dayjs';

const moneyFormatter = new Intl.NumberFormat('mk-MK', { style: 'currency', currency: 'MKD', maximumFractionDigits: 0 });

export function formatMoney(amount: number): string {
    return moneyFormatter.format(amount);
}

export function formatDateTime(date: string): string {
    return dayjs(date).format('DD.MM.YYYY HH:mm');
}
