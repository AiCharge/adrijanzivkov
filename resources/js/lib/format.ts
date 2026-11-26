export function formatAmount(amount: number): string {
    return new Intl.NumberFormat('mk-MK').format(amount);
}

export function stripHtml(html: string): string {
    if (typeof document === 'undefined') {
        return html
            .replace(/<[^>]*>/g, ' ')
            .replace(/\s+/g, ' ')
            .trim();
    }

    const div = document.createElement('div');
    div.innerHTML = html;

    return div.textContent || div.innerText || '';
}
